// Generated macro for impl_1653 (impl)
macro_rules! Depcrate_syncimpl_1653 {
() => {
// Module: crate::sync
// Provides: {"impl_1653"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized + fmt :: Display , A : Allocator > fmt :: Display for UniqueArc < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& * * self , f) } }
};
}
