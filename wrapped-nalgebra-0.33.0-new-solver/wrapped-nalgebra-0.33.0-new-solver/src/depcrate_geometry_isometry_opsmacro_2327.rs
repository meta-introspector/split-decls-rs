// Generated macro for macro_2327 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2327 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2327"}
// Dependencies: {}
md_assign_impl_all ! (MulAssign , mul_assign where T : SimdRealField for T :: Element : SimdRealField ; (U2 , U2) , (U2 , U2) const ; for ; where ; self : Isometry < T , UnitComplex < T >, 2 >, rhs : UnitComplex < T >; [val] => self . rotation *= rhs ; [ref] => self . rotation *= rhs . clone () ;) ;
};
}
