// Generated macro for impl_221 (impl)
macro_rules! Depcrate_integerimpl_221 {
() => {
// Module: crate::integer
// Provides: {"impl_221"}
// Dependencies: {}
impl < B : Buffer > fmt :: Display for IntegerLit < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , &* self . raw) } }
};
}
