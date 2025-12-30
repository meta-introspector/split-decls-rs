// Generated macro for impl_957 (impl)
macro_rules! Depcrate_tokenimpl_957 {
() => {
// Module: crate::token
// Provides: {"impl_957"}
// Dependencies: {}
impl fmt :: Display for ResetToken { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for byte in self . iter () { write ! (f , "{byte:02x}") ? ; } Ok (()) } }
};
}
