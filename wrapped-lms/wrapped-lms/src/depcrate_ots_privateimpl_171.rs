// Generated macro for impl_171 (impl)
macro_rules! Depcrate_ots_privateimpl_171 {
() => {
// Module: crate::ots::private
// Provides: {"impl_171"}
// Dependencies: {}
impl < Mode : LmsOtsMode > RandomizedSignerMut < Signature < Mode > > for SigningKey < Mode > { fn try_sign_with_rng < R : TryCryptoRng + ? Sized > (& mut self , rng : & mut R , msg : & [u8] ,) -> Result < Signature < Mode > , Error > { self . try_multipart_sign_with_rng (rng , & [msg]) } }
};
}
