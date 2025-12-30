// Generated macro for impl_18 (impl)
macro_rules! Depcrate_ffi_utilimpl_18 {
() => {
// Module: crate::ffi_util
// Provides: {"impl_18"}
// Dependencies: {}
impl CStrLike for CString { type Baked = CString ; type Error = std :: convert :: Infallible ; fn bake (self) -> Result < Self :: Baked , Self :: Error > { Ok (self) } fn into_c_string (self) -> Result < CString , Self :: Error > { Ok (self) } }
};
}
