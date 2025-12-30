// Generated macro for macro_1367 (macro)
macro_rules! Depcrate_geometry_point_opsmacro_1367 {
() => {
// Module: crate::geometry::point_ops
// Provides: {"macro_1367"}
// Dependencies: {}
md_impl_all ! (Mul , mul ; (Const < R1 >, Const < C1 >) , (Const < D2 >, U1) const D2 , R1 , C1 ; for SA ; where SA : Storage < T , Const < R1 >, Const < C1 >>, ShapeConstraint : AreMultipliable < Const < R1 >, Const < C1 >, Const < D2 >, U1 >; self : Matrix < T , Const < R1 >, Const < C1 >, SA >, right : Point < T , D2 >, Output = Point < T , R1 >; [val val] => Point :: from (self * right . coords) ; [ref val] => Point :: from (self * right . coords) ; [val ref] => Point :: from (self * & right . coords) ; [ref ref] => Point :: from (self * & right . coords) ;) ;
};
}
