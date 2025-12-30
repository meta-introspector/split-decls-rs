// Generated macro for impl_7 (impl)
macro_rules! Depcrate_stateimpl_7 {
() => {
// Module: crate::state
// Provides: {"impl_7"}
// Dependencies: {}
impl Data { # [doc = " Create new durable transaction nonce data."] pub fn new (authority : Pubkey , durable_nonce : DurableNonce , lamports_per_signature : u64 ,) -> Self { Data { authority , durable_nonce , fee_calculator : FeeCalculator :: new (lamports_per_signature) , } } # [doc = " Hash value used as recent_blockhash field in Transactions."] # [doc = " Named blockhash for legacy reasons, but durable nonce and blockhash"] # [doc = " have separate domains."] pub fn blockhash (& self) -> Hash { self . durable_nonce . 0 } # [doc = " Get the cost per signature for the next transaction to use this nonce."] pub fn get_lamports_per_signature (& self) -> u64 { self . fee_calculator . lamports_per_signature } }
};
}
