// Generated macro for impl_1655 (impl)
macro_rules! Depcrate_syncimpl_1655 {
() => {
// Module: crate::sync
// Provides: {"impl_1655"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized , A : Allocator > fmt :: Pointer for UniqueArc < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Pointer :: fmt (& (& raw const * * self) , f) } }
};
}
