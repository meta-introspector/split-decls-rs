// Generated macro for gauss_step_swap (function)
macro_rules! Depcrate_linalg_lugauss_step_swap {
() => {
// Module: crate::linalg::lu
// Provides: {"gauss_step_swap"}
// Dependencies: {}
# [doc (hidden)] # [doc = " Swaps the rows `i` with the row `piv` and executes one step of gaussian elimination on the i-th"] # [doc = " row and column of `matrix`. The diagonal element `matrix[(i, i)]` is provided as argument."] pub fn gauss_step_swap < T , R : Dim , C : Dim , S > (matrix : & mut Matrix < T , R , C , S > , diag : T , i : usize , piv : usize ,) where T : Scalar + Field , S : StorageMut < T , R , C > , { let piv = piv - i ; let mut submat = matrix . view_range_mut (i .. , i ..) ; let inv_diag = T :: one () / diag ; let (mut coeffs , mut submat) = submat . columns_range_pair_mut (0 , 1 ..) ; coeffs . swap ((0 , 0) , (piv , 0)) ; let mut coeffs = coeffs . rows_range_mut (1 ..) ; coeffs *= inv_diag ; let (mut pivot_row , mut down) = submat . rows_range_pair_mut (0 , 1 ..) ; for k in 0 .. pivot_row . ncols () { mem :: swap (& mut pivot_row [k] , & mut down [(piv - 1 , k)]) ; down . column_mut (k) . axpy (- pivot_row [k] . clone () , & coeffs , T :: one ()) ; } }
};
}
