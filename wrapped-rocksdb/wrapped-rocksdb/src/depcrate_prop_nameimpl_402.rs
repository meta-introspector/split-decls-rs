// Generated macro for impl_402 (impl)
macro_rules! Depcrate_prop_nameimpl_402 {
() => {
// Module: crate::prop_name
// Provides: {"impl_402"}
// Dependencies: {}
impl < 'a > CStrLike for & 'a PropertyName { type Baked = & 'a CStr ; type Error = std :: convert :: Infallible ; # [inline] fn bake (self) -> Result < Self :: Baked , Self :: Error > { Ok (self . as_c_str ()) } # [inline] fn into_c_string (self) -> Result < CString , Self :: Error > { Ok (self . 0 . clone ()) } }
};
}
