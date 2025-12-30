// Generated macro for unbalance (function)
macro_rules! Depcrate_linalg_balancingunbalance {
() => {
// Module: crate::linalg::balancing
// Provides: {"unbalance"}
// Dependencies: {}
# [doc = " Computes in-place `D * m * D.inverse()`, where `D` is the matrix with diagonal `d`."] pub fn unbalance < T : RealField , D : Dim > (m : & mut OMatrix < T , D , D > , d : & OVector < T , D >) where DefaultAllocator : Allocator < D , D > + Allocator < D > , { assert ! (m . is_square () , "Unable to unbalance a non-square matrix.") ; assert_eq ! (m . nrows () , d . len () , "Unbalancing: mismatched dimensions.") ; for j in 0 .. d . len () { let mut col = m . column_mut (j) ; let denom = T :: one () / d [j] . clone () ; for i in 0 .. d . len () { col [i] *= d [i] . clone () * denom . clone () ; } } }
};
}
