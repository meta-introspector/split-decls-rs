// Generated macro for impl_16 (impl)
macro_rules! Depcrate_ffi_utilimpl_16 {
() => {
// Module: crate::ffi_util
// Provides: {"impl_16"}
// Dependencies: {}
impl CStrLike for & String { type Baked = CString ; type Error = std :: ffi :: NulError ; fn bake (self) -> Result < Self :: Baked , Self :: Error > { CString :: new (self . as_bytes ()) } fn into_c_string (self) -> Result < CString , Self :: Error > { CString :: new (self . as_bytes ()) } }
};
}
