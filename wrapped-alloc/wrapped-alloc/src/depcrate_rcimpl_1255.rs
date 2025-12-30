// Generated macro for impl_1255 (impl)
macro_rules! Depcrate_rcimpl_1255 {
() => {
// Module: crate::rc
// Provides: {"impl_1255"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized , A : Allocator > fmt :: Pointer for Rc < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Pointer :: fmt (& (& raw const * * self) , f) } }
};
}
