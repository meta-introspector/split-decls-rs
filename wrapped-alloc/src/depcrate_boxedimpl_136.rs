// Generated macro for impl_136 (impl)
macro_rules! Depcrate_boxedimpl_136 {
() => {
// Module: crate::boxed
// Provides: {"impl_136"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : fmt :: Debug + ? Sized , A : Allocator > fmt :: Debug for Box < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
