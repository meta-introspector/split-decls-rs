// Generated macro for impl_86 (impl)
macro_rules! Depcrate_boxedimpl_86 {
() => {
// Module: crate::boxed
// Provides: {"impl_86"}
// Dependencies: {}
impl < T : fmt :: Display + ? Sized , A : Allocator > fmt :: Display for Box < T , A > { # [inline (always)] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& * * self , f) } }
};
}
