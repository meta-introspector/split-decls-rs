// Generated macro for impl_15 (impl)
macro_rules! Depcrate_lms_keypairimpl_15 {
() => {
// Module: crate::lms::keypair
// Provides: {"impl_15"}
// Dependencies: {}
impl < Mode : LmsMode > Keypair for SigningKey < Mode > { type VerifyingKey = VerifyingKey < Mode > ; fn verifying_key (& self) -> Self :: VerifyingKey { self . public () } }
};
}
