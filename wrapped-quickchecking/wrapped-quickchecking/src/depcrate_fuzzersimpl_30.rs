// Generated macro for impl_30 (impl)
macro_rules! Depcrate_fuzzersimpl_30 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_30"}
// Dependencies: {}
# [doc = " Enables to string and format for `DeclarationC` types."] impl fmt :: Display for DeclarationC { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { DeclarationC :: FunctionPtrDecl (ref d) => write ! (f , "{d}") , DeclarationC :: StructDecl (ref d) => write ! (f , "{d}") , DeclarationC :: UnionDecl (ref d) => write ! (f , "{d}") , DeclarationC :: VariableDecl (ref d) => write ! (f , "{d}") , DeclarationC :: FunctionDecl (ref d) => write ! (f , "{d}") , } } }
};
}
