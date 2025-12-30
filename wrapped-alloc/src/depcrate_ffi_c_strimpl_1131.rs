// Generated macro for impl_1131 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1131 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1131"}
// Dependencies: {}
# [stable (feature = "cow_from_cstr" , since = "1.28.0")] impl < 'a > From < & 'a CString > for Cow < 'a , CStr > { # [doc = " Converts a `&`[`CString`] into a borrowed [`Cow`] without copying or allocating."] # [inline] fn from (s : & 'a CString) -> Cow < 'a , CStr > { Cow :: Borrowed (s . as_c_str ()) } }
};
}
