// Generated macro for impl_1302 (impl)
macro_rules! Depcrate_rcimpl_1302 {
() => {
// Module: crate::rc
// Provides: {"impl_1302"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized + fmt :: Debug , A : Allocator > fmt :: Debug for UniqueRc < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
