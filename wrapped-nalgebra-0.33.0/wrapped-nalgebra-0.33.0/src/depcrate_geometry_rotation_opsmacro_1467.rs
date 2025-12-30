// Generated macro for macro_1467 (macro)
macro_rules! Depcrate_geometry_rotation_opsmacro_1467 {
() => {
// Module: crate::geometry::rotation_ops
// Provides: {"macro_1467"}
// Dependencies: {}
md_impl_all ! (Mul , mul ; (Const < D >, Const < D >) , (Const < D >, U1) const D ; for ; where ShapeConstraint : AreMultipliable < Const < D >, Const < D >, Const < D >, U1 >; self : Rotation < T , D >, right : Point < T , D >, Output = Point < T , D >; [val val] => self . into_inner () * right ; [ref val] => self . matrix () * right ; [val ref] => self . into_inner () * right ; [ref ref] => self . matrix () * right ;) ;
};
}
