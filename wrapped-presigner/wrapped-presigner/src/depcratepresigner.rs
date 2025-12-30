// Generated macro for Presigner (struct)
macro_rules! DepcratePresigner {
() => {
// Module: crate
// Provides: {"Presigner"}
// Dependencies: {}
# [doc = " A `Signer` implementation that represents a `Signature` that has been"] # [doc = " constructed externally. Performs a signature verification against the"] # [doc = " expected message upon `sign()` requests to affirm its relationship to"] # [doc = " the `message` bytes"] # [derive (Clone , Debug , Default)] pub struct Presigner { pubkey : Pubkey , signature : Signature , }
};
}
