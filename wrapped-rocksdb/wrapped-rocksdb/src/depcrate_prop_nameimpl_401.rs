// Generated macro for impl_401 (impl)
macro_rules! Depcrate_prop_nameimpl_401 {
() => {
// Module: crate::prop_name
// Provides: {"impl_401"}
// Dependencies: {}
impl CStrLike for PropertyName { type Baked = CString ; type Error = std :: convert :: Infallible ; # [inline] fn bake (self) -> Result < Self :: Baked , Self :: Error > { Ok (self . 0) } # [inline] fn into_c_string (self) -> Result < CString , Self :: Error > { Ok (self . 0) } }
};
}
