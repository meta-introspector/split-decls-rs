// Generated macro for impl_115 (impl)
macro_rules! Depcrate_string_cowimpl_115 {
() => {
// Module: crate::string_cow
// Provides: {"impl_115"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > std :: str :: FromStr for KStringCowBase < '_ , B > { type Err = std :: convert :: Infallible ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (Self :: from_string (s . into ())) } }
};
}
