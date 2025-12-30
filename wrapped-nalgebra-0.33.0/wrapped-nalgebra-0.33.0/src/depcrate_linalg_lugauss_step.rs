// Generated macro for gauss_step (function)
macro_rules! Depcrate_linalg_lugauss_step {
() => {
// Module: crate::linalg::lu
// Provides: {"gauss_step"}
// Dependencies: {}
# [doc (hidden)] # [doc = " Executes one step of gaussian elimination on the i-th row and column of `matrix`. The diagonal"] # [doc = " element `matrix[(i, i)]` is provided as argument."] pub fn gauss_step < T , R : Dim , C : Dim , S > (matrix : & mut Matrix < T , R , C , S > , diag : T , i : usize) where T : Scalar + Field , S : StorageMut < T , R , C > , { let mut submat = matrix . view_range_mut (i .. , i ..) ; let inv_diag = T :: one () / diag ; let (mut coeffs , mut submat) = submat . columns_range_pair_mut (0 , 1 ..) ; let mut coeffs = coeffs . rows_range_mut (1 ..) ; coeffs *= inv_diag ; let (pivot_row , mut down) = submat . rows_range_pair_mut (0 , 1 ..) ; for k in 0 .. pivot_row . ncols () { down . column_mut (k) . axpy (- pivot_row [k] . clone () , & coeffs , T :: one ()) ; } }
};
}
