// Generated macro for VerifierResult (type)
macro_rules! Depcrate_verifierVerifierResult {
() => {
// Module: crate::verifier
// Provides: {"VerifierResult"}
// Dependencies: {}
# [doc = " Result of a verification operation."] # [doc = ""] # [doc = " Unlike `VerifierStepResult` which may be `Ok` while still having reported"] # [doc = " errors, this type always returns `Err` if an error (fatal or not) was reported."] pub type VerifierResult < T > = Result < T , VerifierErrors > ;
};
}
