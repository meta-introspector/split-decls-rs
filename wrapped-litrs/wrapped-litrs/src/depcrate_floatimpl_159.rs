// Generated macro for impl_159 (impl)
macro_rules! Depcrate_floatimpl_159 {
() => {
// Module: crate::float
// Provides: {"impl_159"}
// Dependencies: {}
impl < B : Buffer > fmt :: Display for FloatLit < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , &* self . raw) } }
};
}
