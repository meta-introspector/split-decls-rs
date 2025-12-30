// Generated macro for impl_1962 (impl)
macro_rules! Depcrate_vecimpl_1962 {
() => {
// Module: crate::vec
// Provides: {"impl_1962"}
// Dependencies: {}
# [doc = " Implements ordering of vectors, [lexicographically](Ord#lexicographical-comparison)."] # [stable (feature = "rust1" , since = "1.0.0")] impl < T : Ord , A : Allocator > Ord for Vec < T , A > { # [inline] fn cmp (& self , other : & Self) -> Ordering { Ord :: cmp (& * * self , & * * other) } }
};
}
