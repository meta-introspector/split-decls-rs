// Generated macro for impl_1130 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1130 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1130"}
// Dependencies: {}
# [stable (feature = "cow_from_cstr" , since = "1.28.0")] impl < 'a > From < & 'a CStr > for Cow < 'a , CStr > { # [doc = " Converts a [`CStr`] into a borrowed [`Cow`] without copying or allocating."] # [inline] fn from (s : & 'a CStr) -> Cow < 'a , CStr > { Cow :: Borrowed (s) } }
};
}
