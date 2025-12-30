// Generated macro for impl_83 (impl)
macro_rules! Depcrate_ppcimpl_83 {
() => {
// Module: crate::ppc
// Provides: {"impl_83"}
// Dependencies: {}
impl < F : FloatConvert < Fallback < F > > > fmt :: Display for DoubleFloat < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& Fallback :: from (* self) , f) } }
};
}
