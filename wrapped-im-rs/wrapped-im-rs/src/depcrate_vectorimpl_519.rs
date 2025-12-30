// Generated macro for impl_519 (impl)
macro_rules! Depcrate_vectorimpl_519 {
() => {
// Module: crate::vector
// Provides: {"impl_519"}
// Dependencies: {}
impl < A : Clone > Clone for Vector < A > { # [doc = " Clone a vector."] # [doc = ""] # [doc = " Time: O(1), or O(n) with a very small, bounded *n* for an inline vector."] fn clone (& self) -> Self { Self { vector : match & self . vector { Inline (pool , chunk) => Inline (pool . clone () , chunk . clone ()) , Single (pool , chunk) => Single (pool . clone () , chunk . clone ()) , Full (pool , tree) => Full (pool . clone () , tree . clone ()) , } , } } }
};
}
