// Generated macro for impl_76 (impl)
macro_rules! Depcrate_lms_publicimpl_76 {
() => {
// Module: crate::lms::public
// Provides: {"impl_76"}
// Dependencies: {}
impl < Mode : LmsMode > MultipartVerifier < Signature < Mode > > for VerifyingKey < Mode > { fn multipart_verify (& self , msg : & [& [u8]] , signature : & Signature < Mode >) -> Result < () , Error > { let key_candidate = signature . lmots_sig . raw_recover_pubkey (self . id , signature . q , msg) ; let mut node_num = signature . q + Mode :: LEAVES ; let mut tmp = Mode :: Hasher :: new () . chain_update (self . id) . chain_update (node_num . to_be_bytes ()) . chain_update (D_LEAF) . chain_update (key_candidate . k) . finalize () ; for i in 0 .. Mode :: H { let mut hasher = Mode :: Hasher :: new () . chain_update (self . id) . chain_update ((node_num / 2) . to_be_bytes ()) . chain_update (D_INTR) ; if node_num % 2 == 1 { hasher . update (& signature . path [i]) ; hasher . update (& tmp) ; } else { hasher . update (& tmp) ; hasher . update (& signature . path [i]) ; } hasher . finalize_into (& mut tmp) ; node_num /= 2 ; } if self . k == tmp { Ok (()) } else { Err (Error :: new ()) } } }
};
}
