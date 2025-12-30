// Generated macro for impl_209 (impl)
macro_rules! Depcrate_astimpl_209 {
() => {
// Module: crate::ast
// Provides: {"impl_209"}
// Dependencies: {}
impl UseTree { pub fn ident (& self) -> Ident { match self . kind { UseTreeKind :: Simple (Some (rename)) => rename , UseTreeKind :: Simple (None) => { self . prefix . segments . last () . expect ("empty prefix in a simple import") . ident } _ => panic ! ("`UseTree::ident` can only be used on a simple import") , } } }
};
}
