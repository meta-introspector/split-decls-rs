// Generated macro for impl_19 (impl)
macro_rules! Depcrate_boxedimpl_19 {
() => {
// Module: crate::boxed
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a , T : fmt :: Display + ? Sized > fmt :: Display for Box < 'a , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& * * self , f) } }
};
}
