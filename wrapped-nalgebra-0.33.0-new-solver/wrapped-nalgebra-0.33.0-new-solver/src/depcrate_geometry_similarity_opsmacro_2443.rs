// Generated macro for macro_2443 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2443 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2443"}
// Dependencies: {}
md_assign_impl_all ! (MulAssign , mul_assign where T : SimdRealField for T :: Element : SimdRealField ; (U3 , U3) , (U3 , U3) const ; for ; where ; self : Similarity < T , UnitQuaternion < T >, 3 >, rhs : UnitQuaternion < T >; [val] => self . isometry . rotation *= rhs ; [ref] => self . isometry . rotation *= rhs . clone () ;) ;
};
}
