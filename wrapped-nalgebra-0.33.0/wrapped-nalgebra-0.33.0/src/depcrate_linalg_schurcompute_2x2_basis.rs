// Generated macro for compute_2x2_basis (function)
macro_rules! Depcrate_linalg_schurcompute_2x2_basis {
() => {
// Module: crate::linalg::schur
// Provides: {"compute_2x2_basis"}
// Dependencies: {}
# [doc = " Computes the singular vectors for a 2x2 matrix."] # [doc = ""] # [doc = " Returns `None` if the matrix has complex eigenvalues, or is upper-triangular. In both case,"] # [doc = " the basis is the identity."] fn compute_2x2_basis < T : ComplexField , S : Storage < T , U2 , U2 > > (m : & SquareMatrix < T , U2 , S > ,) -> Option < GivensRotation < T > > { let h10 = m [(1 , 0)] . clone () ; if h10 . is_zero () { return None ; } if let Some ((eigval1 , eigval2)) = compute_2x2_eigvals (m) { let x1 = eigval1 - m [(1 , 1)] . clone () ; let x2 = eigval2 - m [(1 , 1)] . clone () ; if x1 . clone () . norm1 () > x2 . clone () . norm1 () { Some (GivensRotation :: new (x1 , h10) . 0) } else { Some (GivensRotation :: new (x2 , h10) . 0) } } else { None } }
};
}
