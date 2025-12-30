// Generated macro for macro_1468 (macro)
macro_rules! Depcrate_geometry_rotation_opsmacro_1468 {
() => {
// Module: crate::geometry::rotation_ops
// Provides: {"macro_1468"}
// Dependencies: {}
md_impl_all ! (Mul , mul ; (Const < D >, Const < D >) , (Const < D >, U1) const D ; for S ; where S : Storage < T , Const < D >>, ShapeConstraint : AreMultipliable < Const < D >, Const < D >, Const < D >, U1 >; self : Rotation < T , D >, right : Unit < Vector < T , Const < D >, S >>, Output = Unit < SVector < T , D >>; [val val] => Unit :: new_unchecked (self . into_inner () * right . into_inner ()) ; [ref val] => Unit :: new_unchecked (self . matrix () * right . into_inner ()) ; [val ref] => Unit :: new_unchecked (self . into_inner () * right . as_ref ()) ; [ref ref] => Unit :: new_unchecked (self . matrix () * right . as_ref ()) ;) ;
};
}
