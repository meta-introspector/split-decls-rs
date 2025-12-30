// Generated macro for impl_131 (impl)
macro_rules! Depcrate_boxedimpl_131 {
() => {
// Module: crate::boxed
// Provides: {"impl_131"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + Ord , A : Allocator > Ord for Box < T , A > { # [inline] fn cmp (& self , other : & Self) -> Ordering { Ord :: cmp (& * * self , & * * other) } }
};
}
