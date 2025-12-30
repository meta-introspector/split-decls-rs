// Generated macro for impl_1303 (impl)
macro_rules! Depcrate_rcimpl_1303 {
() => {
// Module: crate::rc
// Provides: {"impl_1303"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized , A : Allocator > fmt :: Pointer for UniqueRc < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Pointer :: fmt (& (& raw const * * self) , f) } }
};
}
