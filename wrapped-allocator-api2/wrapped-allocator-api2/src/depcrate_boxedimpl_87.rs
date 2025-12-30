// Generated macro for impl_87 (impl)
macro_rules! Depcrate_boxedimpl_87 {
() => {
// Module: crate::boxed
// Provides: {"impl_87"}
// Dependencies: {}
impl < T : fmt :: Debug + ? Sized , A : Allocator > fmt :: Debug for Box < T , A > { # [inline (always)] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
