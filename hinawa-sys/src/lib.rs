// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal)]

extern crate libc;
extern crate glib_sys as glib;
extern crate gobject_sys as gobject;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type HinawaFwRcode = c_int;
pub const HINAWA_FW_RCODE_COMPLETE: HinawaFwRcode = 0;
pub const HINAWA_FW_RCODE_CONFLICT_ERROR: HinawaFwRcode = 4;
pub const HINAWA_FW_RCODE_DATA_ERROR: HinawaFwRcode = 5;
pub const HINAWA_FW_RCODE_TYPE_ERROR: HinawaFwRcode = 6;
pub const HINAWA_FW_RCODE_ADDRESS_ERROR: HinawaFwRcode = 7;
pub const HINAWA_FW_RCODE_SEND_ERROR: HinawaFwRcode = 16;
pub const HINAWA_FW_RCODE_CANCELLED: HinawaFwRcode = 17;
pub const HINAWA_FW_RCODE_BUSY: HinawaFwRcode = 18;
pub const HINAWA_FW_RCODE_GENERATION: HinawaFwRcode = 19;
pub const HINAWA_FW_RCODE_NO_ACK: HinawaFwRcode = 20;

pub type HinawaFwTcode = c_int;
pub const HINAWA_FW_TCODE_WRITE_QUADLET_REQUEST: HinawaFwTcode = 0;
pub const HINAWA_FW_TCODE_WRITE_BLOCK_REQUEST: HinawaFwTcode = 1;
pub const HINAWA_FW_TCODE_WRITE_RESPONSE: HinawaFwTcode = 2;
pub const HINAWA_FW_TCODE_READ_QUADLET_REQUEST: HinawaFwTcode = 4;
pub const HINAWA_FW_TCODE_READ_BLOCK_REQUEST: HinawaFwTcode = 5;
pub const HINAWA_FW_TCODE_READ_QUADLET_RESPONSE: HinawaFwTcode = 6;
pub const HINAWA_FW_TCODE_READ_BLOCK_RESPONSE: HinawaFwTcode = 7;
pub const HINAWA_FW_TCODE_CYCLE_START: HinawaFwTcode = 8;
pub const HINAWA_FW_TCODE_LOCK_REQUEST: HinawaFwTcode = 9;
pub const HINAWA_FW_TCODE_STREAM_DATA: HinawaFwTcode = 10;
pub const HINAWA_FW_TCODE_LOCK_RESPONSE: HinawaFwTcode = 11;
pub const HINAWA_FW_TCODE_LOCK_MASK_SWAP: HinawaFwTcode = 17;
pub const HINAWA_FW_TCODE_LOCK_COMPARE_SWAP: HinawaFwTcode = 18;
pub const HINAWA_FW_TCODE_LOCK_FETCH_ADD: HinawaFwTcode = 19;
pub const HINAWA_FW_TCODE_LOCK_LITTLE_ADD: HinawaFwTcode = 20;
pub const HINAWA_FW_TCODE_LOCK_BOUNDED_ADD: HinawaFwTcode = 21;
pub const HINAWA_FW_TCODE_LOCK_WRAP_ADD: HinawaFwTcode = 22;
pub const HINAWA_FW_TCODE_LOCK_VENDOR_DEPENDENT: HinawaFwTcode = 23;

pub type HinawaSndUnitType = c_int;
pub const HINAWA_SND_UNIT_TYPE_DICE: HinawaSndUnitType = 1;
pub const HINAWA_SND_UNIT_TYPE_FIREWORKS: HinawaSndUnitType = 2;
pub const HINAWA_SND_UNIT_TYPE_BEBOB: HinawaSndUnitType = 3;
pub const HINAWA_SND_UNIT_TYPE_OXFW: HinawaSndUnitType = 4;
pub const HINAWA_SND_UNIT_TYPE_DIGI00X: HinawaSndUnitType = 5;
pub const HINAWA_SND_UNIT_TYPE_TASCAM: HinawaSndUnitType = 6;
pub const HINAWA_SND_UNIT_TYPE_MOTU: HinawaSndUnitType = 7;
pub const HINAWA_SND_UNIT_TYPE_FIREFACE: HinawaSndUnitType = 8;

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaFwFcpClass {
    pub parent_class: HinawaFwRespClass,
}

impl ::std::fmt::Debug for HinawaFwFcpClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwFcpClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct _HinawaFwFcpPrivate(c_void);

pub type HinawaFwFcpPrivate = *mut _HinawaFwFcpPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaFwNodeClass {
    pub parent_class: gobject::GObjectClass,
    pub bus_update: Option<unsafe extern "C" fn(*mut HinawaFwNode)>,
    pub disconnected: Option<unsafe extern "C" fn(*mut HinawaFwNode)>,
}

impl ::std::fmt::Debug for HinawaFwNodeClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwNodeClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("bus_update", &self.bus_update)
         .field("disconnected", &self.disconnected)
         .finish()
    }
}

#[repr(C)]
pub struct _HinawaFwNodePrivate(c_void);

pub type HinawaFwNodePrivate = *mut _HinawaFwNodePrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaFwReqClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for HinawaFwReqClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwReqClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct _HinawaFwReqPrivate(c_void);

pub type HinawaFwReqPrivate = *mut _HinawaFwReqPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaFwRespClass {
    pub parent_class: gobject::GObjectClass,
    pub requested: Option<unsafe extern "C" fn(*mut HinawaFwResp, HinawaFwTcode) -> HinawaFwRcode>,
}

impl ::std::fmt::Debug for HinawaFwRespClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwRespClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("requested", &self.requested)
         .finish()
    }
}

#[repr(C)]
pub struct _HinawaFwRespPrivate(c_void);

pub type HinawaFwRespPrivate = *mut _HinawaFwRespPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaSndDg00xClass {
    pub parent_class: HinawaSndUnitClass,
    pub message: Option<unsafe extern "C" fn(*mut HinawaSndDg00x, u32)>,
}

impl ::std::fmt::Debug for HinawaSndDg00xClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaSndDg00xClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("message", &self.message)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaSndDiceClass {
    pub parent_class: HinawaSndUnitClass,
    pub notified: Option<unsafe extern "C" fn(*mut HinawaSndDice, c_uint)>,
}

impl ::std::fmt::Debug for HinawaSndDiceClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaSndDiceClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("notified", &self.notified)
         .finish()
    }
}

#[repr(C)]
pub struct _HinawaSndDicePrivate(c_void);

pub type HinawaSndDicePrivate = *mut _HinawaSndDicePrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaSndEfwClass {
    pub parent_class: HinawaSndUnitClass,
}

impl ::std::fmt::Debug for HinawaSndEfwClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaSndEfwClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct _HinawaSndEfwPrivate(c_void);

pub type HinawaSndEfwPrivate = *mut _HinawaSndEfwPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaSndMotuClass {
    pub parent_class: HinawaSndUnitClass,
    pub notified: Option<unsafe extern "C" fn(*mut HinawaSndMotu, c_uint)>,
}

impl ::std::fmt::Debug for HinawaSndMotuClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaSndMotuClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("notified", &self.notified)
         .finish()
    }
}

#[repr(C)]
pub struct _HinawaSndMotuPrivate(c_void);

pub type HinawaSndMotuPrivate = *mut _HinawaSndMotuPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaSndTscmClass {
    pub parent_class: HinawaSndUnitClass,
    pub control: Option<unsafe extern "C" fn(*mut HinawaSndTscm, c_uint, c_uint, c_uint)>,
}

impl ::std::fmt::Debug for HinawaSndTscmClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaSndTscmClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("control", &self.control)
         .finish()
    }
}

#[repr(C)]
pub struct _HinawaSndTscmPrivate(c_void);

pub type HinawaSndTscmPrivate = *mut _HinawaSndTscmPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaSndUnitClass {
    pub parent_class: gobject::GObjectClass,
    pub lock_status: Option<unsafe extern "C" fn(*mut HinawaSndUnit, gboolean)>,
    pub disconnected: Option<unsafe extern "C" fn(*mut HinawaSndUnit)>,
}

impl ::std::fmt::Debug for HinawaSndUnitClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaSndUnitClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("lock_status", &self.lock_status)
         .field("disconnected", &self.disconnected)
         .finish()
    }
}

#[repr(C)]
pub struct _HinawaSndUnitPrivate(c_void);

pub type HinawaSndUnitPrivate = *mut _HinawaSndUnitPrivate;

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaFwFcp {
    pub parent_instance: HinawaFwResp,
    pub priv_: *mut HinawaFwFcpPrivate,
}

impl ::std::fmt::Debug for HinawaFwFcp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwFcp @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaFwNode {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut HinawaFwNodePrivate,
}

impl ::std::fmt::Debug for HinawaFwNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwNode @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaFwReq {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut HinawaFwReqPrivate,
}

impl ::std::fmt::Debug for HinawaFwReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwReq @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaFwResp {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut HinawaFwRespPrivate,
}

impl ::std::fmt::Debug for HinawaFwResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwResp @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaSndDg00x {
    pub parent_instance: HinawaSndUnit,
}

impl ::std::fmt::Debug for HinawaSndDg00x {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaSndDg00x @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaSndDice {
    pub parent_instance: HinawaSndUnit,
    pub priv_: *mut HinawaSndDicePrivate,
}

impl ::std::fmt::Debug for HinawaSndDice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaSndDice @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaSndEfw {
    pub parent_instance: HinawaSndUnit,
    pub priv_: *mut HinawaSndEfwPrivate,
}

impl ::std::fmt::Debug for HinawaSndEfw {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaSndEfw @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaSndMotu {
    pub parent_instance: HinawaSndUnit,
    pub priv_: *mut HinawaSndMotuPrivate,
}

impl ::std::fmt::Debug for HinawaSndMotu {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaSndMotu @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaSndTscm {
    pub parent_instance: HinawaSndUnit,
    pub priv_: *mut HinawaSndTscmPrivate,
}

impl ::std::fmt::Debug for HinawaSndTscm {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaSndTscm @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HinawaSndUnit {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut HinawaSndUnitPrivate,
}

impl ::std::fmt::Debug for HinawaSndUnit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaSndUnit @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .field("priv_", &self.priv_)
         .finish()
    }
}

extern "C" {

    //=========================================================================
    // HinawaFwRcode
    //=========================================================================
    pub fn hinawa_fw_rcode_get_type() -> GType;

    //=========================================================================
    // HinawaFwTcode
    //=========================================================================
    pub fn hinawa_fw_tcode_get_type() -> GType;

    //=========================================================================
    // HinawaSndUnitType
    //=========================================================================
    pub fn hinawa_snd_unit_type_get_type() -> GType;

    //=========================================================================
    // HinawaFwFcp
    //=========================================================================
    pub fn hinawa_fw_fcp_get_type() -> GType;
    pub fn hinawa_fw_fcp_new() -> *mut HinawaFwFcp;
    pub fn hinawa_fw_fcp_bind(self_: *mut HinawaFwFcp, node: *mut HinawaFwNode, error: *mut *mut glib::GError);
    pub fn hinawa_fw_fcp_transaction(self_: *mut HinawaFwFcp, req_frame: *const u8, req_frame_size: size_t, resp_frame: *const *mut u8, resp_frame_size: *mut size_t, error: *mut *mut glib::GError);
    pub fn hinawa_fw_fcp_unbind(self_: *mut HinawaFwFcp);

    //=========================================================================
    // HinawaFwNode
    //=========================================================================
    pub fn hinawa_fw_node_get_type() -> GType;
    pub fn hinawa_fw_node_new() -> *mut HinawaFwNode;
    pub fn hinawa_fw_node_create_source(self_: *mut HinawaFwNode, gsrc: *mut *mut glib::GSource, error: *mut *mut glib::GError);
    pub fn hinawa_fw_node_get_config_rom(self_: *mut HinawaFwNode, image: *mut *const u8, length: *mut size_t, error: *mut *mut glib::GError);
    pub fn hinawa_fw_node_open(self_: *mut HinawaFwNode, path: *const c_char, error: *mut *mut glib::GError);

    //=========================================================================
    // HinawaFwReq
    //=========================================================================
    pub fn hinawa_fw_req_get_type() -> GType;
    pub fn hinawa_fw_req_new() -> *mut HinawaFwReq;
    pub fn hinawa_fw_req_transaction(self_: *mut HinawaFwReq, node: *mut HinawaFwNode, tcode: HinawaFwTcode, addr: u64, length: size_t, frame: *const *mut u8, frame_size: *mut size_t, error: *mut *mut glib::GError);

    //=========================================================================
    // HinawaFwResp
    //=========================================================================
    pub fn hinawa_fw_resp_get_type() -> GType;
    pub fn hinawa_fw_resp_new() -> *mut HinawaFwResp;
    pub fn hinawa_fw_resp_get_req_frame(self_: *mut HinawaFwResp, frame: *mut *const u8, length: *mut size_t);
    pub fn hinawa_fw_resp_release(self_: *mut HinawaFwResp);
    pub fn hinawa_fw_resp_reserve(self_: *mut HinawaFwResp, node: *mut HinawaFwNode, addr: u64, width: c_uint, error: *mut *mut glib::GError);
    pub fn hinawa_fw_resp_set_resp_frame(self_: *mut HinawaFwResp, frame: *mut u8, length: size_t);

    //=========================================================================
    // HinawaSndDg00x
    //=========================================================================
    pub fn hinawa_snd_dg00x_get_type() -> GType;
    pub fn hinawa_snd_dg00x_new() -> *mut HinawaSndDg00x;
    pub fn hinawa_snd_dg00x_open(self_: *mut HinawaSndDg00x, path: *mut c_char, error: *mut *mut glib::GError);

    //=========================================================================
    // HinawaSndDice
    //=========================================================================
    pub fn hinawa_snd_dice_get_type() -> GType;
    pub fn hinawa_snd_dice_new() -> *mut HinawaSndDice;
    pub fn hinawa_snd_dice_open(self_: *mut HinawaSndDice, path: *mut c_char, error: *mut *mut glib::GError);
    pub fn hinawa_snd_dice_transaction(self_: *mut HinawaSndDice, addr: u64, frame: *const u32, frame_count: size_t, bit_flag: u32, error: *mut *mut glib::GError);

    //=========================================================================
    // HinawaSndEfw
    //=========================================================================
    pub fn hinawa_snd_efw_get_type() -> GType;
    pub fn hinawa_snd_efw_new() -> *mut HinawaSndEfw;
    pub fn hinawa_snd_efw_open(self_: *mut HinawaSndEfw, path: *mut c_char, error: *mut *mut glib::GError);
    pub fn hinawa_snd_efw_transaction(self_: *mut HinawaSndEfw, category: c_uint, command: c_uint, args: *const u32, arg_count: size_t, params: *const *mut u32, param_count: *mut size_t, error: *mut *mut glib::GError);

    //=========================================================================
    // HinawaSndMotu
    //=========================================================================
    pub fn hinawa_snd_motu_get_type() -> GType;
    pub fn hinawa_snd_motu_new() -> *mut HinawaSndMotu;
    pub fn hinawa_snd_motu_open(self_: *mut HinawaSndMotu, path: *mut c_char, error: *mut *mut glib::GError);

    //=========================================================================
    // HinawaSndTscm
    //=========================================================================
    pub fn hinawa_snd_tscm_get_type() -> GType;
    pub fn hinawa_snd_tscm_new() -> *mut HinawaSndTscm;
    pub fn hinawa_snd_tscm_get_state(self_: *mut HinawaSndTscm, error: *mut *mut glib::GError) -> *const [u32; 64];
    pub fn hinawa_snd_tscm_open(self_: *mut HinawaSndTscm, path: *mut c_char, error: *mut *mut glib::GError);

    //=========================================================================
    // HinawaSndUnit
    //=========================================================================
    pub fn hinawa_snd_unit_get_type() -> GType;
    pub fn hinawa_snd_unit_new() -> *mut HinawaSndUnit;
    pub fn hinawa_snd_unit_create_source(self_: *mut HinawaSndUnit, gsrc: *mut *mut glib::GSource, error: *mut *mut glib::GError);
    pub fn hinawa_snd_unit_get_node(self_: *mut HinawaSndUnit, node: *mut *mut HinawaFwNode);
    pub fn hinawa_snd_unit_lock(self_: *mut HinawaSndUnit, error: *mut *mut glib::GError);
    pub fn hinawa_snd_unit_open(self_: *mut HinawaSndUnit, path: *mut c_char, error: *mut *mut glib::GError);
    pub fn hinawa_snd_unit_unlock(self_: *mut HinawaSndUnit, error: *mut *mut glib::GError);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn hinawa_sigs_marshal_ENUM__ENUM(closure: *mut gobject::GClosure, return_value: *mut gobject::GValue, n_param_values: c_uint, param_values: *const gobject::GValue, invocation_hint: gpointer, marshal_data: gpointer);
    pub fn hinawa_sigs_marshal_VOID__UINT_UINT_UINT(closure: *mut gobject::GClosure, return_value: *mut gobject::GValue, n_param_values: c_uint, param_values: *const gobject::GValue, invocation_hint: gpointer, marshal_data: gpointer);

}