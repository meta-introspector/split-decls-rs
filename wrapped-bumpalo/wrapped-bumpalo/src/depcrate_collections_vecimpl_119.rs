// Generated macro for impl_119 (impl)
macro_rules! Depcrate_collections_vecimpl_119 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_119"}
// Dependencies: {}
# [doc = " Implements ordering of vectors, lexicographically."] impl < 'bump , T : 'bump + Ord > Ord for Vec < 'bump , T > { # [inline] fn cmp (& self , other : & Vec < 'bump , T >) -> Ordering { Ord :: cmp (& * * self , & * * other) } }
};
}
