// Generated macro for macro_2445 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2445 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2445"}
// Dependencies: {}
md_assign_impl_all ! (MulAssign , mul_assign where T : SimdRealField for T :: Element : SimdRealField ; (U2 , U2) , (U2 , U2) const ; for ; where ; self : Similarity < T , UnitComplex < T >, 2 >, rhs : UnitComplex < T >; [val] => self . isometry . rotation *= rhs ; [ref] => self . isometry . rotation *= rhs . clone () ;) ;
};
}
