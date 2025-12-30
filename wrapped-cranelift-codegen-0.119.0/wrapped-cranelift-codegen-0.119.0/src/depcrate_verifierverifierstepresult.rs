// Generated macro for VerifierStepResult (type)
macro_rules! Depcrate_verifierVerifierStepResult {
() => {
// Module: crate::verifier
// Provides: {"VerifierStepResult"}
// Dependencies: {}
# [doc = " Result of a step in the verification process."] # [doc = ""] # [doc = " Functions that return `VerifierStepResult` should also take a"] # [doc = " mutable reference to `VerifierErrors` as argument in order to report"] # [doc = " errors."] # [doc = ""] # [doc = " Here, `Ok` represents a step that **did not lead to a fatal error**,"] # [doc = " meaning that the verification process may continue. However, other (non-fatal)"] # [doc = " errors might have been reported through the previously mentioned `VerifierErrors`"] # [doc = " argument."] pub type VerifierStepResult = Result < () , () > ;
};
}
