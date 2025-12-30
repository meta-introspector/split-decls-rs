// Generated macro for impl_61 (impl)
macro_rules! Depcrate_fuzzersimpl_61 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_61"}
// Dependencies: {}
# [doc = " Enables to string and format for `HeaderC` types."] impl fmt :: Display for HeaderC { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for decl in & self . def . decls { write ! (f , "{decl}") ? ; } Ok (()) } }
};
}
