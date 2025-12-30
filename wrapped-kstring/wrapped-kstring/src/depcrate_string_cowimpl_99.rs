// Generated macro for impl_99 (impl)
macro_rules! Depcrate_string_cowimpl_99 {
() => {
// Module: crate::string_cow
// Provides: {"impl_99"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > fmt :: Display for KStringCowBase < '_ , B > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self . as_str () , f) } }
};
}
