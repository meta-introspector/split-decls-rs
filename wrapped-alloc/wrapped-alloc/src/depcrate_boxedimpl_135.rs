// Generated macro for impl_135 (impl)
macro_rules! Depcrate_boxedimpl_135 {
() => {
// Module: crate::boxed
// Provides: {"impl_135"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : fmt :: Display + ? Sized , A : Allocator > fmt :: Display for Box < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& * * self , f) } }
};
}
