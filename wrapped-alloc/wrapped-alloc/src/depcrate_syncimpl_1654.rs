// Generated macro for impl_1654 (impl)
macro_rules! Depcrate_syncimpl_1654 {
() => {
// Module: crate::sync
// Provides: {"impl_1654"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized + fmt :: Debug , A : Allocator > fmt :: Debug for UniqueArc < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
