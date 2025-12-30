// Generated macro for impl_1960 (impl)
macro_rules! Depcrate_vecimpl_1960 {
() => {
// Module: crate::vec
// Provides: {"impl_1960"}
// Dependencies: {}
# [doc = " Implements comparison of vectors, [lexicographically](Ord#lexicographical-comparison)."] # [stable (feature = "rust1" , since = "1.0.0")] impl < T , A1 , A2 > PartialOrd < Vec < T , A2 > > for Vec < T , A1 > where T : PartialOrd , A1 : Allocator , A2 : Allocator , { # [inline] fn partial_cmp (& self , other : & Vec < T , A2 >) -> Option < Ordering > { PartialOrd :: partial_cmp (& * * self , & * * other) } }
};
}
