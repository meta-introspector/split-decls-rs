// Generated macro for impl_98 (impl)
macro_rules! Depcrate_string_cowimpl_98 {
() => {
// Module: crate::string_cow
// Provides: {"impl_98"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > fmt :: Debug for KStringCowBase < '_ , B > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . as_str () . fmt (f) } }
};
}
