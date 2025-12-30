// Generated macro for impl_15 (impl)
macro_rules! Depcrate_ffi_utilimpl_15 {
() => {
// Module: crate::ffi_util
// Provides: {"impl_15"}
// Dependencies: {}
impl CStrLike for & str { type Baked = CString ; type Error = std :: ffi :: NulError ; fn bake (self) -> Result < Self :: Baked , Self :: Error > { CString :: new (self) } fn into_c_string (self) -> Result < CString , Self :: Error > { CString :: new (self) } }
};
}
