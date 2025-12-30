// Generated macro for impl_1301 (impl)
macro_rules! Depcrate_rcimpl_1301 {
() => {
// Module: crate::rc
// Provides: {"impl_1301"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized + fmt :: Display , A : Allocator > fmt :: Display for UniqueRc < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& * * self , f) } }
};
}
