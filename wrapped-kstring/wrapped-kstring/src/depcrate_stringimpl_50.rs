// Generated macro for impl_50 (impl)
macro_rules! Depcrate_stringimpl_50 {
() => {
// Module: crate::string
// Provides: {"impl_50"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > fmt :: Debug for KStringBase < B > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . as_str () . fmt (f) } }
};
}
