// Generated macro for SigningState (struct)
macro_rules! Depcrate_ed25519SigningState {
() => {
// Module: crate::ed25519
// Provides: {"SigningState"}
// Dependencies: {}
# [doc = " The state of a streaming signature operation."] # [derive (Clone)] pub struct SigningState { hasher : sha512 :: Hash , az : [u8 ; 64] , nonce : [u8 ; 64] , }
};
}
