// Generated macro for verify_function (function)
macro_rules! Depcrate_verifierverify_function {
() => {
// Module: crate::verifier
// Provides: {"verify_function"}
// Dependencies: {}
# [doc = " Verify `func`."] pub fn verify_function < 'a , FOI : Into < FlagsOrIsa < 'a > > > (func : & Function , fisa : FOI ,) -> VerifierResult < () > { let _tt = timing :: verifier () ; let mut errors = VerifierErrors :: default () ; let verifier = Verifier :: new (func , fisa . into ()) ; let result = verifier . run (& mut errors) ; if errors . is_empty () { result . unwrap () ; Ok (()) } else { Err (errors) } }
};
}
