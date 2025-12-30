// Generated macro for impl_29 (impl)
macro_rules! Depcrate_fuzzersimpl_29 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_29"}
// Dependencies: {}
# [doc = " A qucickcheck trait for describing how `DeclarationC` types can be"] # [doc = " randomly generated and shrunk."] impl Arbitrary for DeclarationC { fn arbitrary (g : & mut Gen) -> DeclarationC { match gen_range (g , 0 , 5) { 0 => DeclarationC :: FunctionDecl (FunctionPrototypeC :: arbitrary (g)) , 1 => DeclarationC :: FunctionPtrDecl (FunctionPointerDeclarationC :: arbitrary (g) ,) , 2 => DeclarationC :: StructDecl (StructDeclarationC :: arbitrary (g)) , 3 => DeclarationC :: UnionDecl (UnionDeclarationC :: arbitrary (g)) , 4 => { DeclarationC :: VariableDecl (BasicTypeDeclarationC :: arbitrary (g)) } _ => unreachable ! () , } } }
};
}
