// Generated macro for impl_2840 (impl)
macro_rules! Depcrate_verifierimpl_2840 {
() => {
// Module: crate::verifier
// Provides: {"impl_2840"}
// Dependencies: {}
impl Display for VerifierErrors { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { for err in & self . 0 { writeln ! (f , "- {err}") ? ; } Ok (()) } }
};
}
