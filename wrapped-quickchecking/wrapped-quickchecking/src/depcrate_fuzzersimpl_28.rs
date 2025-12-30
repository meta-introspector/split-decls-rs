// Generated macro for impl_28 (impl)
macro_rules! Depcrate_fuzzersimpl_28 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_28"}
// Dependencies: {}
# [doc = " `MakeUnique` is used in generation of C headers to make `DeclarationC`"] # [doc = " identifiers unique."] impl MakeUnique for DeclarationC { fn make_unique (& mut self , stamp : usize) { match * self { DeclarationC :: FunctionDecl (ref mut d) => d . make_unique (stamp) , DeclarationC :: FunctionPtrDecl (ref mut d) => d . make_unique (stamp) , DeclarationC :: StructDecl (ref mut d) => d . make_unique (stamp) , DeclarationC :: UnionDecl (ref mut d) => d . make_unique (stamp) , DeclarationC :: VariableDecl (ref mut d) => d . make_unique (stamp) , } } }
};
}
