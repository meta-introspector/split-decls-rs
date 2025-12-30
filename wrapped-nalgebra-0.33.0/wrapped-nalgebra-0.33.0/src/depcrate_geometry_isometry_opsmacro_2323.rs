// Generated macro for macro_2323 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2323 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2323"}
// Dependencies: {}
md_assign_impl_all ! (MulAssign , mul_assign where T : SimdRealField for T :: Element : SimdRealField ; (Const < D >, U1) , (Const < D >, Const < D >) const D ; for ; where ; self : Isometry < T , Rotation < T , D >, D >, rhs : Rotation < T , D >; [val] => self . rotation *= rhs ; [ref] => self . rotation *= rhs . clone () ;) ;
};
}
