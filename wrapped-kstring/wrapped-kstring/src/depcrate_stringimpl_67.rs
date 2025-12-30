// Generated macro for impl_67 (impl)
macro_rules! Depcrate_stringimpl_67 {
() => {
// Module: crate::string
// Provides: {"impl_67"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > std :: str :: FromStr for KStringBase < B > { type Err = std :: convert :: Infallible ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (Self :: from_ref (s)) } }
};
}
