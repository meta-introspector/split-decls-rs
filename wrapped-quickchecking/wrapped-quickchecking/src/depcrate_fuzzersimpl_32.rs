// Generated macro for impl_32 (impl)
macro_rules! Depcrate_fuzzersimpl_32 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_32"}
// Dependencies: {}
# [doc = " Enables to string and format for `DeclarationListC` types."] impl fmt :: Display for DeclarationListC { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for decl in & self . decls { write ! (f , "{decl}") ? ; } Ok (()) } }
};
}
