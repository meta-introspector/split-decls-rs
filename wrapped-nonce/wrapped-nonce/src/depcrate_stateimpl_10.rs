// Generated macro for impl_10 (impl)
macro_rules! Depcrate_stateimpl_10 {
() => {
// Module: crate::state
// Provides: {"impl_10"}
// Dependencies: {}
impl State { # [doc = " Create new durable transaction nonce state."] pub fn new_initialized (authority : & Pubkey , durable_nonce : DurableNonce , lamports_per_signature : u64 ,) -> Self { Self :: Initialized (Data :: new (* authority , durable_nonce , lamports_per_signature)) } # [doc = " Get the serialized size of the nonce state."] pub const fn size () -> usize { 80 } }
};
}
