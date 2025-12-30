// Generated macro for cumsum (function)
macro_rules! Depcrate_sparse_cs_utilscumsum {
() => {
// Module: crate::sparse::cs_utils
// Provides: {"cumsum"}
// Dependencies: {}
pub fn cumsum < D : Dim > (a : & mut OVector < usize , D > , b : & mut OVector < usize , D >) -> usize where DefaultAllocator : Allocator < D > , { assert ! (a . len () == b . len ()) ; let mut sum = 0 ; for i in 0 .. a . len () { b [i] = sum ; sum += a [i] ; a [i] = b [i] ; } sum }
};
}
