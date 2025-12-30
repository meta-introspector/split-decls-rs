// Generated macro for impl_59 (impl)
macro_rules! Depcrate_arcimpl_59 {
() => {
// Module: crate::arc
// Provides: {"impl_59"}
// Dependencies: {}
impl < T : ? Sized + fmt :: Debug > fmt :: Debug for Arc < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
