// Generated macro for impl_1254 (impl)
macro_rules! Depcrate_rcimpl_1254 {
() => {
// Module: crate::rc
// Provides: {"impl_1254"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + fmt :: Debug , A : Allocator > fmt :: Debug for Rc < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
