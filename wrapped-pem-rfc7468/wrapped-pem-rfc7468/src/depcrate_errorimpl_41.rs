// Generated macro for impl_41 (impl)
macro_rules! Depcrate_errorimpl_41 {
() => {
// Module: crate::error
// Provides: {"impl_41"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Error :: Base64 (err) => write ! (f , "PEM Base64 error: {err}") , Error :: CharacterEncoding => f . write_str ("PEM character encoding error") , Error :: EncapsulatedText => f . write_str ("PEM error in encapsulated text") , Error :: HeaderDisallowed => f . write_str ("PEM headers disallowed by RFC7468") , Error :: Label => f . write_str ("PEM type label invalid") , Error :: Length => f . write_str ("PEM length invalid") , Error :: Preamble => f . write_str ("PEM preamble contains invalid data (NUL byte)") , Error :: PreEncapsulationBoundary => { f . write_str ("PEM error in pre-encapsulation boundary") } Error :: PostEncapsulationBoundary => { f . write_str ("PEM error in post-encapsulation boundary") } Error :: UnexpectedTypeLabel { expected } => { write ! (f , "unexpected PEM type label: expecting \"BEGIN {expected}\"") } } } }
};
}
