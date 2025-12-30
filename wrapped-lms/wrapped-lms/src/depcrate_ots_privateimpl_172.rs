// Generated macro for impl_172 (impl)
macro_rules! Depcrate_ots_privateimpl_172 {
() => {
// Module: crate::ots::private
// Provides: {"impl_172"}
// Dependencies: {}
impl < Mode : LmsOtsMode > RandomizedMultipartSignerMut < Signature < Mode > > for SigningKey < Mode > { fn try_multipart_sign_with_rng < R : TryCryptoRng + ? Sized > (& mut self , rng : & mut R , msg : & [& [u8]] ,) -> Result < Signature < Mode > , Error > { if ! self . valid { return Err (Error :: from_source (LmsOtsInvalidPrivateKey { })) ; } let mut c = < Output < Mode :: Hasher > > :: default () ; rng . try_fill_bytes (& mut c) . map_err (| _ | Error :: new ()) ? ; let mut q_hasher = Mode :: Hasher :: new () ; q_hasher . update (self . id) ; q_hasher . update (self . q . to_be_bytes ()) ; q_hasher . update (D_MESG) ; q_hasher . update (& c) ; msg . iter () . for_each (| slice | q_hasher . update (slice)) ; let q = q_hasher . finalize () ; let y = Mode :: expand (& q) . into_iter () . enumerate () . map (| (i , a) | { let a = a as u32 ; let mut tmp = self . x [i] . clone () ; for j in 0 .. a { Mode :: Hasher :: new () . chain_update (self . id) . chain_update (self . q . to_be_bytes ()) . chain_update ((i as u16) . to_be_bytes ()) . chain_update ((j as u8) . to_be_bytes ()) . chain_update (& tmp) . finalize_into (& mut tmp) ; } tmp }) ; let y = Array :: from_iter (y) ; let sig = Signature { c , y } ; self . q . zeroize () ; self . id . zeroize () ; self . x . zeroize () ; self . valid = false ; Ok (sig) } }
};
}
