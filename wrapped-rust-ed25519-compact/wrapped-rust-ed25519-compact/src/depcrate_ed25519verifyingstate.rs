// Generated macro for VerifyingState (struct)
macro_rules! Depcrate_ed25519VerifyingState {
() => {
// Module: crate::ed25519
// Provides: {"VerifyingState"}
// Dependencies: {}
# [doc = " The state of a streaming verification operation."] # [derive (Clone)] pub struct VerifyingState { hasher : sha512 :: Hash , signature : Signature , a : GeP3 , }
};
}
