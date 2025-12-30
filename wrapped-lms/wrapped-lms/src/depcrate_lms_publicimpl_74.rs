// Generated macro for impl_74 (impl)
macro_rules! Depcrate_lms_publicimpl_74 {
() => {
// Module: crate::lms::public
// Provides: {"impl_74"}
// Dependencies: {}
impl < Mode : LmsMode > VerifyingKey < Mode > { pub fn new (id : Identifier , k : Output < Mode :: Hasher >) -> Self { Self { id , k } } # [doc = " Returns the 16-byte identifier of the public key"] pub fn id (& self) -> & Identifier { & self . id } # [doc = " Returns the N-byte public key as a byte slice"] pub fn k (& self) -> & [u8] { & self . k } }
};
}
