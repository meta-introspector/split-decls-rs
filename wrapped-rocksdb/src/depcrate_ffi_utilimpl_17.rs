// Generated macro for impl_17 (impl)
macro_rules! Depcrate_ffi_utilimpl_17 {
() => {
// Module: crate::ffi_util
// Provides: {"impl_17"}
// Dependencies: {}
impl CStrLike for & CStr { type Baked = Self ; type Error = std :: convert :: Infallible ; fn bake (self) -> Result < Self :: Baked , Self :: Error > { Ok (self) } fn into_c_string (self) -> Result < CString , Self :: Error > { Ok (self . to_owned ()) } }
};
}
