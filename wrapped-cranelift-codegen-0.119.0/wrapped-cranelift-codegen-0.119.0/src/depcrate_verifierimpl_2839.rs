// Generated macro for impl_2839 (impl)
macro_rules! Depcrate_verifierimpl_2839 {
() => {
// Module: crate::verifier
// Provides: {"impl_2839"}
// Dependencies: {}
impl Into < VerifierResult < () > > for VerifierErrors { fn into (self) -> VerifierResult < () > { if self . is_empty () { Ok (()) } else { Err (self) } } }
};
}
