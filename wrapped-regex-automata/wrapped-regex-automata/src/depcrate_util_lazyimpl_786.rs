// Generated macro for impl_786 (impl)
macro_rules! Depcrate_util_lazyimpl_786 {
() => {
// Module: crate::util::lazy
// Provides: {"impl_786"}
// Dependencies: {}
impl < T : fmt :: Debug , F : Fn () -> T > fmt :: Debug for Lazy < T , F > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . 0 . fmt (f) } }
};
}
