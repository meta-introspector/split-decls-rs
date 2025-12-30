// Generated macro for impl_95 (impl)
macro_rules! Depcrate_string_cowimpl_95 {
() => {
// Module: crate::string_cow
// Provides: {"impl_95"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > Ord for KStringCowBase < '_ , B > { # [inline] fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . as_str () . cmp (other . as_str ()) } }
};
}
