// Generated macro for impl_58 (impl)
macro_rules! Depcrate_arcimpl_58 {
() => {
// Module: crate::arc
// Provides: {"impl_58"}
// Dependencies: {}
impl < T : ? Sized + fmt :: Display > fmt :: Display for Arc < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& * * self , f) } }
};
}
