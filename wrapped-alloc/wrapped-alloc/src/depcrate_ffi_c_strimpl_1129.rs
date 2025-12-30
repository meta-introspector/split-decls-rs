// Generated macro for impl_1129 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1129 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1129"}
// Dependencies: {}
# [stable (feature = "cow_from_cstr" , since = "1.28.0")] impl < 'a > From < CString > for Cow < 'a , CStr > { # [doc = " Converts a [`CString`] into an owned [`Cow`] without copying or allocating."] # [inline] fn from (s : CString) -> Cow < 'a , CStr > { Cow :: Owned (s) } }
};
}
