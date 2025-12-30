// Generated macro for impl_128 (impl)
macro_rules! Depcrate_agreementimpl_128 {
() => {
// Module: crate::agreement
// Provides: {"impl_128"}
// Dependencies: {}
impl < B : AsRef < [u8] > > TryFrom < & UnparsedPublicKey < B > > for ParsedPublicKey { type Error = KeyRejected ; fn try_from (upk : & UnparsedPublicKey < B >) -> Result < Self , Self :: Error > { upk . parse () } }
};
}
