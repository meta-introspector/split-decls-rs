// Generated macro for macro_2441 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2441 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2441"}
// Dependencies: {}
md_assign_impl_all ! (MulAssign , mul_assign where T : SimdRealField for T :: Element : SimdRealField ; (Const < D >, U1) , (Const < D >, Const < D >) const D ; for ; where ; self : Similarity < T , Rotation < T , D >, D >, rhs : Rotation < T , D >; [val] => self . isometry . rotation *= rhs ; [ref] => self . isometry . rotation *= rhs . clone () ;) ;
};
}
