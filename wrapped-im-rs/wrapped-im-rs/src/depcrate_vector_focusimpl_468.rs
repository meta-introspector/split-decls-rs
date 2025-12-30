// Generated macro for impl_468 (impl)
macro_rules! Depcrate_vector_focusimpl_468 {
() => {
// Module: crate::vector::focus
// Provides: {"impl_468"}
// Dependencies: {}
impl < 'a , A > Clone for Focus < 'a , A > where A : Clone + 'a , { fn clone (& self) -> Self { match self { Focus :: Single (chunk) => Focus :: Single (chunk) , Focus :: Full (tree) => Focus :: Full (tree . clone ()) , } } }
};
}
