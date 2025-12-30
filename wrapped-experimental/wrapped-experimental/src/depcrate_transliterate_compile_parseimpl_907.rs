// Generated macro for impl_907 (impl)
macro_rules! Depcrate_transliterate_compile_parseimpl_907 {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"impl_907"}
// Dependencies: {}
impl Display for BasicId { fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "{}-{}" , self . source . to_ascii_lowercase () , self . target . to_ascii_lowercase ()) ? ; if let Some (variant) = & self . variant { write ! (f , "/{}" , variant . to_ascii_lowercase ()) ? ; } Ok (()) } }
};
}
