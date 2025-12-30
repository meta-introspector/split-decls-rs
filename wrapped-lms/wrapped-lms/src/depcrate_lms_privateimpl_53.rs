// Generated macro for impl_53 (impl)
macro_rules! Depcrate_lms_privateimpl_53 {
() => {
// Module: crate::lms::private
// Provides: {"impl_53"}
// Dependencies: {}
impl < Mode : LmsMode > RandomizedMultipartSignerMut < Signature < Mode > > for SigningKey < Mode > { fn try_multipart_sign_with_rng < R : TryCryptoRng + ? Sized > (& mut self , rng : & mut R , msg : & [& [u8]] ,) -> Result < Signature < Mode > , Error > { if self . q >= Mode :: LEAVES { return Err (Error :: from_source (LmsOutOfPrivateKeys { })) ; } let mut ots_priv_key = OtsPrivateKey :: < Mode :: OtsMode > :: new_from_seed (self . q , self . id , & self . seed) ; let ots_sig = ots_priv_key . try_multipart_sign_with_rng (rng , msg) ? ; let r = (1 << Mode :: H) + self . q ; let auth_path = (0 .. Mode :: H) . map (| i | self . auth_tree [(((r >> i) ^ 1) - 1) as usize] . clone ()) ; self . q += 1 ; Ok (Signature :: < Mode > { q : self . q - 1 , lmots_sig : ots_sig , path : auth_path . collect () , }) } }
};
}
