// Generated macro for compute_2x2_eigvals (function)
macro_rules! Depcrate_linalg_schurcompute_2x2_eigvals {
() => {
// Module: crate::linalg::schur
// Provides: {"compute_2x2_eigvals"}
// Dependencies: {}
fn compute_2x2_eigvals < T : ComplexField , S : Storage < T , U2 , U2 > > (m : & SquareMatrix < T , U2 , S > ,) -> Option < (T , T) > { let h00 = m [(0 , 0)] . clone () ; let h10 = m [(1 , 0)] . clone () ; let h01 = m [(0 , 1)] . clone () ; let h11 = m [(1 , 1)] . clone () ; let val = (h00 . clone () - h11 . clone ()) * crate :: convert (0.5) ; let discr = h10 * h01 + val . clone () * val ; discr . try_sqrt () . map (| sqrt_discr | { let half_tra = (h00 + h11) * crate :: convert (0.5) ; (half_tra . clone () + sqrt_discr . clone () , half_tra - sqrt_discr) }) }
};
}
