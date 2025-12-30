// Generated macro for impl_259 (impl)
macro_rules! Depcrate_vecimpl_259 {
() => {
// Module: crate::vec
// Provides: {"impl_259"}
// Dependencies: {}
# [doc = " Implements comparison of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison)."] impl < T : PartialOrd , A : Allocator > PartialOrd for Vec < T , A > { # [inline (always)] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { PartialOrd :: partial_cmp (& * * self , & * * other) } }
};
}
