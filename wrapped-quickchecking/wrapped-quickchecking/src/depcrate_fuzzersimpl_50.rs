// Generated macro for impl_50 (impl)
macro_rules! Depcrate_fuzzersimpl_50 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_50"}
// Dependencies: {}
# [doc = " `MakeUnique` is used in generation of C headers to make"] # [doc = " `FunctionPointerDeclarationC` identifiers unique."] impl MakeUnique for FunctionPointerDeclarationC { fn make_unique (& mut self , stamp : usize) { let _ = write ! (self . ident_id , "_{stamp}") ; } }
};
}
