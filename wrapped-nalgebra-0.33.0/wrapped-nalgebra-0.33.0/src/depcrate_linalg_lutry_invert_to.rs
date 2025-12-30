// Generated macro for try_invert_to (function)
macro_rules! Depcrate_linalg_lutry_invert_to {
() => {
// Module: crate::linalg::lu
// Provides: {"try_invert_to"}
// Dependencies: {}
# [doc = " Performs a LU decomposition to overwrite `out` with the inverse of `matrix`."] # [doc = ""] # [doc = " If `matrix` is not invertible, `false` is returned and `out` may contain invalid data."] pub fn try_invert_to < T : ComplexField , D : Dim , S > (mut matrix : OMatrix < T , D , D > , out : & mut Matrix < T , D , D , S > ,) -> bool where S : StorageMut < T , D , D > , DefaultAllocator : Allocator < D , D > , { assert ! (matrix . is_square () , "LU inversion: unable to invert a rectangular matrix.") ; let dim = matrix . nrows () ; out . fill_with_identity () ; for i in 0 .. dim { let piv = matrix . view_range (i .. , i) . icamax () + i ; let diag = matrix [(piv , i)] . clone () ; if diag . is_zero () { return false ; } if piv != i { out . swap_rows (i , piv) ; matrix . columns_range_mut (.. i) . swap_rows (i , piv) ; gauss_step_swap (& mut matrix , diag , i , piv) ; } else { gauss_step (& mut matrix , diag , i) ; } } let _ = matrix . solve_lower_triangular_with_diag_mut (out , T :: one ()) ; matrix . solve_upper_triangular_mut (out) }
};
}
