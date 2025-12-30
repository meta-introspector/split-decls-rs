// Generated macro for impl_127 (impl)
macro_rules! Depcrate_agreementimpl_127 {
() => {
// Module: crate::agreement
// Provides: {"impl_127"}
// Dependencies: {}
impl < B : AsRef < [u8] > > UnparsedPublicKey < B > { # [allow (dead_code)] fn parse (& self) -> Result < ParsedPublicKey , KeyRejected > { ParsedPublicKey :: new (& self . bytes , self . alg . id . nid ()) } }
};
}
