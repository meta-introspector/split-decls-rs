// Generated macro for impl_261 (impl)
macro_rules! Depcrate_vecimpl_261 {
() => {
// Module: crate::vec
// Provides: {"impl_261"}
// Dependencies: {}
# [doc = " Implements ordering of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison)."] impl < T : Ord , A : Allocator > Ord for Vec < T , A > { # [inline (always)] fn cmp (& self , other : & Self) -> Ordering { Ord :: cmp (& * * self , & * * other) } }
};
}
