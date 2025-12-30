// Generated macro for macro_1469 (macro)
macro_rules! Depcrate_geometry_rotation_opsmacro_1469 {
() => {
// Module: crate::geometry::rotation_ops
// Provides: {"macro_1469"}
// Dependencies: {}
md_assign_impl_all ! (MulAssign , mul_assign ; (Const < D >, Const < D >) , (Const < D >, Const < D >) const D ; for ; where ; self : Rotation < T , D >, right : Rotation < T , D >; [val] => self . matrix_mut_unchecked () . mul_assign (right . into_inner ()) ; [ref] => self . matrix_mut_unchecked () . mul_assign (right . matrix ()) ;) ;
};
}
