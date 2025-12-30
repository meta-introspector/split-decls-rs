// Generated macro for impl_1115 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1115 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1115"}
// Dependencies: {}
# [doc = " Delegates to the [`CStr`] implementation of [`fmt::Debug`],"] # [doc = " showing invalid UTF-8 as hex escapes."] # [stable (feature = "rust1" , since = "1.0.0")] impl fmt :: Debug for CString { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . as_c_str () , f) } }
};
}
