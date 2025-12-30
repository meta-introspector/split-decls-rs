// Generated macro for impl_1965 (impl)
macro_rules! Depcrate_vecimpl_1965 {
() => {
// Module: crate::vec
// Provides: {"impl_1965"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for Vec < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
