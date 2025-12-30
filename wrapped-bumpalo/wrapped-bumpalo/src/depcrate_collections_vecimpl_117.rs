// Generated macro for impl_117 (impl)
macro_rules! Depcrate_collections_vecimpl_117 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_117"}
// Dependencies: {}
# [doc = " Implements comparison of vectors, lexicographically."] impl < 'bump , T : 'bump + PartialOrd > PartialOrd for Vec < 'bump , T > { # [inline] fn partial_cmp (& self , other : & Vec < 'bump , T >) -> Option < Ordering > { PartialOrd :: partial_cmp (& * * self , & * * other) } }
};
}
