// Generated macro for decompose_2x2 (function)
macro_rules! Depcrate_linalg_schurdecompose_2x2 {
() => {
// Module: crate::linalg::schur
// Provides: {"decompose_2x2"}
// Dependencies: {}
fn decompose_2x2 < T : ComplexField , D : Dim > (mut m : OMatrix < T , D , D > , compute_q : bool ,) -> Option < (Option < OMatrix < T , D , D > > , OMatrix < T , D , D >) > where DefaultAllocator : Allocator < D , D > , { let dim = m . shape_generic () . 0 ; let mut q = None ; match compute_2x2_basis (& m . fixed_view :: < 2 , 2 > (0 , 0)) { Some (rot) => { let mut m = m . fixed_view_mut :: < 2 , 2 > (0 , 0) ; let inv_rot = rot . inverse () ; inv_rot . rotate (& mut m) ; rot . rotate_rows (& mut m) ; m [(1 , 0)] = T :: zero () ; if compute_q { let c = T :: from_real (rot . c ()) ; q = Some (OMatrix :: from_column_slice_generic (dim , dim , & [c . clone () , rot . s () , - rot . s () . conjugate () , c] ,)) ; } } None => { if compute_q { q = Some (OMatrix :: identity_generic (dim , dim)) ; } } } ; Some ((q , m)) }
};
}
