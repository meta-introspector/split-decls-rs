// Generated macro for impl_113 (impl)
macro_rules! Depcrate_ots_keypairimpl_113 {
() => {
// Module: crate::ots::keypair
// Provides: {"impl_113"}
// Dependencies: {}
impl < Mode : LmsOtsMode > Keypair for SigningKey < Mode > { type VerifyingKey = VerifyingKey < Mode > ; fn verifying_key (& self) -> Self :: VerifyingKey { self . public () } }
};
}
