// Generated macro for macro_2325 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2325 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2325"}
// Dependencies: {}
md_assign_impl_all ! (MulAssign , mul_assign where T : SimdRealField for T :: Element : SimdRealField ; (U3 , U3) , (U3 , U3) const ; for ; where ; self : Isometry < T , UnitQuaternion < T >, 3 >, rhs : UnitQuaternion < T >; [val] => self . rotation *= rhs ; [ref] => self . rotation *= rhs . clone () ;) ;
};
}
