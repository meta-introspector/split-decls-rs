// Generated macro for impl_19 (impl)
macro_rules! Depcrate_ffi_utilimpl_19 {
() => {
// Module: crate::ffi_util
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a > CStrLike for & 'a CString { type Baked = & 'a CStr ; type Error = std :: convert :: Infallible ; fn bake (self) -> Result < Self :: Baked , Self :: Error > { Ok (self) } fn into_c_string (self) -> Result < CString , Self :: Error > { Ok (self . clone ()) } }
};
}
