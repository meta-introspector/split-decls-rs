// Generated macro for impl_1125 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1125 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1125"}
// Dependencies: {}
# [stable (feature = "c_string_from_str" , since = "1.85.0")] impl FromStr for CString { type Err = NulError ; # [doc = " Converts a string `s` into a [`CString`]."] # [doc = ""] # [doc = " This method is equivalent to [`CString::new`]."] # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: new (s) } }
};
}
