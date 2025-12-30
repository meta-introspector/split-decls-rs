// Generated macro for impl_264 (impl)
macro_rules! Depcrate_vecimpl_264 {
() => {
// Module: crate::vec
// Provides: {"impl_264"}
// Dependencies: {}
impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for Vec < T , A > { # [inline (always)] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
