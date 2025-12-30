// Generated macro for svd_ordered3 (function)
macro_rules! Depcrate_linalg_svd3svd_ordered3 {
() => {
// Module: crate::linalg::svd3
// Provides: {"svd_ordered3"}
// Dependencies: {}
pub fn svd_ordered3 < T : RealField > (m : & Matrix3 < T > , compute_u : bool , compute_v : bool , eps : T , niter : usize ,) -> Option < SVD < T , U3 , U3 > > { let s = m . tr_mul (m) ; let mut v = s . try_symmetric_eigen (eps , niter) ? . eigenvectors ; let mut b = m * & v ; let mut rho0 = b . column (0) . norm_squared () ; let mut rho1 = b . column (1) . norm_squared () ; let mut rho2 = b . column (2) . norm_squared () ; if rho0 < rho1 { b . swap_columns (0 , 1) ; b . column_mut (1) . neg_mut () ; v . swap_columns (0 , 1) ; v . column_mut (1) . neg_mut () ; std :: mem :: swap (& mut rho0 , & mut rho1) ; } if rho0 < rho2 { b . swap_columns (0 , 2) ; b . column_mut (2) . neg_mut () ; v . swap_columns (0 , 2) ; v . column_mut (2) . neg_mut () ; std :: mem :: swap (& mut rho0 , & mut rho2) ; } if rho1 < rho2 { b . swap_columns (1 , 2) ; b . column_mut (2) . neg_mut () ; v . swap_columns (1 , 2) ; v . column_mut (2) . neg_mut () ; std :: mem :: swap (& mut rho0 , & mut rho2) ; } let qr = b . qr () ; Some (SVD { u : if compute_u { Some (qr . q ()) } else { None } , singular_values : qr . diag_internal () . map (| e | e . abs ()) , v_t : if compute_v { Some (v . transpose ()) } else { None } , }) }
};
}
