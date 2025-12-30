// Generated macro for impl_52 (impl)
macro_rules! Depcrate_lms_privateimpl_52 {
() => {
// Module: crate::lms::private
// Provides: {"impl_52"}
// Dependencies: {}
impl < Mode : LmsMode > RandomizedSignerMut < Signature < Mode > > for SigningKey < Mode > { fn try_sign_with_rng < R : TryCryptoRng + ? Sized > (& mut self , rng : & mut R , msg : & [u8] ,) -> Result < Signature < Mode > , Error > { self . try_multipart_sign_with_rng (rng , & [msg]) } }
};
}
