// Generated macro for macro_1470 (macro)
macro_rules! Depcrate_geometry_rotation_opsmacro_1470 {
() => {
// Module: crate::geometry::rotation_ops
// Provides: {"macro_1470"}
// Dependencies: {}
md_assign_impl_all ! (DivAssign , div_assign ; (Const < D >, Const < D >) , (Const < D >, Const < D >) const D ; for ; where ; self : Rotation < T , D >, right : Rotation < T , D >; [val] => self . matrix_mut_unchecked () . mul_assign (right . inverse () . into_inner ()) ; [ref] => self . matrix_mut_unchecked () . mul_assign (right . inverse () . matrix ()) ;) ;
};
}
