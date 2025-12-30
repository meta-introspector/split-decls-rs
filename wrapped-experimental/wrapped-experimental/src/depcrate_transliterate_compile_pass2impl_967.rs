// Generated macro for impl_967 (impl)
macro_rules! Depcrate_transliterate_compile_pass2impl_967 {
() => {
// Module: crate::transliterate::compile::pass2
// Provides: {"impl_967"}
// Dependencies: {}
impl Display for LiteralOrStandin < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match * self { LiteralOrStandin :: Literal (s) => write ! (f , "{s}") , LiteralOrStandin :: Standin (c) => write ! (f , "{c}") , } } }
};
}
