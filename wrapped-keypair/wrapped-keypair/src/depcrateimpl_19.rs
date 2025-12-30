// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl EncodableKeypair for Keypair { type Pubkey = Address ; # [doc = " Returns the associated pubkey. Use this function specifically for settings that involve"] # [doc = " reading or writing pubkeys. For other settings, use `Signer::pubkey()` instead."] fn encodable_pubkey (& self) -> Self :: Pubkey { self . pubkey () } }
};
}
