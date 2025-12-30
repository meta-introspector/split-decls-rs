// Generated macro for impl_1135 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1135 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1135"}
// Dependencies: {}
# [stable (feature = "shared_from_slice2" , since = "1.24.0")] impl From < CString > for Rc < CStr > { # [doc = " Converts a [`CString`] into an <code>[Rc]<[CStr]></code> by moving the [`CString`]"] # [doc = " data into a new [`Rc`] buffer."] # [inline] fn from (s : CString) -> Rc < CStr > { let rc : Rc < [u8] > = Rc :: from (s . into_inner ()) ; unsafe { Rc :: from_raw (Rc :: into_raw (rc) as * const CStr) } } }
};
}
