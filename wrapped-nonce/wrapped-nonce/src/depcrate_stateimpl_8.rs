// Generated macro for impl_8 (impl)
macro_rules! Depcrate_stateimpl_8 {
() => {
// Module: crate::state
// Provides: {"impl_8"}
// Dependencies: {}
impl DurableNonce { pub fn from_blockhash (blockhash : & Hash) -> Self { Self (hashv (& [DURABLE_NONCE_HASH_PREFIX , blockhash . as_ref ()])) } # [doc = " Hash value used as recent_blockhash field in Transactions."] pub fn as_hash (& self) -> & Hash { & self . 0 } }
};
}
