// Generated macro for macro_1462 (macro)
macro_rules! Depcrate_geometry_rotation_opsmacro_1462 {
() => {
// Module: crate::geometry::rotation_ops
// Provides: {"macro_1462"}
// Dependencies: {}
md_impl_all ! (Mul , mul ; (Const < D >, Const < D >) , (Const < D >, Const < D >) const D ; for ; where ; self : Rotation < T , D >, right : Rotation < T , D >, Output = Rotation < T , D >; [val val] => Rotation :: from_matrix_unchecked (self . into_inner () * right . into_inner ()) ; [ref val] => Rotation :: from_matrix_unchecked (self . matrix () * right . into_inner ()) ; [val ref] => Rotation :: from_matrix_unchecked (self . into_inner () * right . matrix ()) ; [ref ref] => Rotation :: from_matrix_unchecked (self . matrix () * right . matrix ()) ;) ;
};
}
