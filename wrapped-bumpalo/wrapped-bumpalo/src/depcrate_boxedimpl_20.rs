// Generated macro for impl_20 (impl)
macro_rules! Depcrate_boxedimpl_20 {
() => {
// Module: crate::boxed
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a , T : fmt :: Debug + ? Sized > fmt :: Debug for Box < 'a , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
