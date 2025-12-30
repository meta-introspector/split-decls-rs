// Generated macro for impl_51 (impl)
macro_rules! Depcrate_stringimpl_51 {
() => {
// Module: crate::string
// Provides: {"impl_51"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > fmt :: Display for KStringBase < B > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self . as_str () , f) } }
};
}
