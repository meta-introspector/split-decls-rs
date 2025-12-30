// Generated macro for macro_1471 (macro)
macro_rules! Depcrate_geometry_rotation_opsmacro_1471 {
() => {
// Module: crate::geometry::rotation_ops
// Provides: {"macro_1471"}
// Dependencies: {}
md_assign_impl_all ! (MulAssign , mul_assign ; (Const < R1 >, Const < C1 >) , (Const < C1 >, Const < C1 >) const R1 , C1 ; for ; where ; self : SMatrix < T , R1 , C1 >, right : Rotation < T , C1 >; [val] => self . mul_assign (right . into_inner ()) ; [ref] => self . mul_assign (right . matrix ()) ;) ;
};
}
