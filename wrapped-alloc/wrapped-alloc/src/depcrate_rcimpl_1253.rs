// Generated macro for impl_1253 (impl)
macro_rules! Depcrate_rcimpl_1253 {
() => {
// Module: crate::rc
// Provides: {"impl_1253"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + fmt :: Display , A : Allocator > fmt :: Display for Rc < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& * * self , f) } }
};
}
