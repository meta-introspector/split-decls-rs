// Generated macro for impl_388 (impl)
macro_rules! Depcrate_prop_nameimpl_388 {
() => {
// Module: crate::prop_name
// Provides: {"impl_388"}
// Dependencies: {}
impl < 'a > CStrLike for & 'a PropName { type Baked = & 'a CStr ; type Error = std :: convert :: Infallible ; # [inline] fn bake (self) -> Result < Self :: Baked , Self :: Error > { Ok (& self . 0) } # [inline] fn into_c_string (self) -> Result < CString , Self :: Error > { Ok (self . 0 . to_owned ()) } }
};
}
