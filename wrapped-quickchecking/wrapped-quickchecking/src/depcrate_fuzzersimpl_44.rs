// Generated macro for impl_44 (impl)
macro_rules! Depcrate_fuzzersimpl_44 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_44"}
// Dependencies: {}
# [doc = " `MakeUnique` is used in generation of C headers to make `StructDeclarationC`"] # [doc = " identifiers unique."] impl MakeUnique for StructDeclarationC { fn make_unique (& mut self , stamp : usize) { let _ = write ! (self . ident_id , "_{stamp}") ; } }
};
}
