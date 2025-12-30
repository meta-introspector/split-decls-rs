// Generated macro for impl_41 (impl)
macro_rules! Depcrate_fuzzersimpl_41 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_41"}
// Dependencies: {}
# [doc = " `MakeUnique` is used in generation of C headers to make `BasicTypeDeclarationC`"] # [doc = " identifiers unique."] impl MakeUnique for BasicTypeDeclarationC { fn make_unique (& mut self , stamp : usize) { let _ = write ! (self . ident_id , "_{stamp}") ; } }
};
}
