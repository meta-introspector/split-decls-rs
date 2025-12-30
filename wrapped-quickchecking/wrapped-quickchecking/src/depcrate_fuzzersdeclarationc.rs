// Generated macro for DeclarationC (enum)
macro_rules! Depcrate_fuzzersDeclarationC {
() => {
// Module: crate::fuzzers
// Provides: {"DeclarationC"}
// Dependencies: {}
# [doc = " `DeclarationC` is used in generation of C headers to represent all supported"] # [doc = " C type declarations allowed in the generated header."] # [derive (Debug , Clone)] pub enum DeclarationC { # [doc = " Function prototype declaration kind."] FunctionDecl (FunctionPrototypeC) , # [doc = " Function pointer declaration kind."] FunctionPtrDecl (FunctionPointerDeclarationC) , # [doc = " Struct declaration kind."] StructDecl (StructDeclarationC) , # [doc = " Union declaration kind."] UnionDecl (UnionDeclarationC) , # [doc = " Basic type declaration kind."] VariableDecl (BasicTypeDeclarationC) , }
};
}
