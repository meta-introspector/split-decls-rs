// Generated macro for vec (function)
macro_rules! Depcrate_vecvec {
() => {
// Module: crate::vec
// Provides: {"vec"}
// Dependencies: {}
# [doc = " Create a new vec from the iterable"] pub fn vec < I > (iterable : I) -> Vec < I :: Item > where I : IntoIterator , { iterable . into_iter () . collect () }
};
}
