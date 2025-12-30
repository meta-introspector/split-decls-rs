// Generated macro for impl_96 (impl)
macro_rules! Depcrate_string_cowimpl_96 {
() => {
// Module: crate::string_cow
// Provides: {"impl_96"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > PartialOrd for KStringCowBase < '_ , B > { # [inline] fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } }
};
}
