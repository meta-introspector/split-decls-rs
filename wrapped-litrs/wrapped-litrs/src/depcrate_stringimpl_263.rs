// Generated macro for impl_263 (impl)
macro_rules! Depcrate_stringimpl_263 {
() => {
// Module: crate::string
// Provides: {"impl_263"}
// Dependencies: {}
impl < B : Buffer > fmt :: Display for StringLit < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (& self . raw) } }
};
}
