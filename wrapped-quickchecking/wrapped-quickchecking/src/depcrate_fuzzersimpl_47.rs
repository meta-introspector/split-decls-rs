// Generated macro for impl_47 (impl)
macro_rules! Depcrate_fuzzersimpl_47 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_47"}
// Dependencies: {}
# [doc = " `MakeUnique` is used in generation of C headers to make `UnionDeclarationC`"] # [doc = " identifiers unique."] impl MakeUnique for UnionDeclarationC { fn make_unique (& mut self , stamp : usize) { let _ = write ! (self . ident_id , "_{stamp}") ; } }
};
}
