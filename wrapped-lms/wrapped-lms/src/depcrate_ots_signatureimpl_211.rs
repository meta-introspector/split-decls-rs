// Generated macro for impl_211 (impl)
macro_rules! Depcrate_ots_signatureimpl_211 {
() => {
// Module: crate::ots::signature
// Provides: {"impl_211"}
// Dependencies: {}
impl < Mode : LmsOtsMode > Signature < Mode > { # [doc = " Returns a public key candidate for this signature as defined by"] # [doc = " algorithm 4b of the LMS RFC. The signature will always be valid for"] # [doc = " the returned public key candidate."] pub fn recover_pubkey (& self , id : Identifier , q : u32 , msg : & [u8]) -> VerifyingKey < Mode > { self . raw_recover_pubkey (id , q , & [msg]) } pub (crate) fn raw_recover_pubkey (& self , id : Identifier , q : u32 , msg : & [& [u8]] ,) -> VerifyingKey < Mode > { let mut msg_hasher = Mode :: Hasher :: new () ; msg_hasher . update (id) ; msg_hasher . update (q . to_be_bytes ()) ; msg_hasher . update (D_MESG) ; msg_hasher . update (& self . c) ; msg . iter () . for_each (| slice | msg_hasher . update (slice)) ; let msg_hash = msg_hasher . finalize () ; let mut hasher = Mode :: Hasher :: new () . chain_update (id) . chain_update (q . to_be_bytes ()) . chain_update (D_PBLC) ; let mut tmp = Array :: default () ; for (i , a) in Mode :: expand (& msg_hash) . into_iter () . enumerate () { tmp . clone_from (& self . y [i]) ; for j in a .. ((1usize << Mode :: W) - 1) as u8 { Mode :: Hasher :: new () . chain_update (id) . chain_update (q . to_be_bytes ()) . chain_update ((i as u16) . to_be_bytes ()) . chain_update (j . to_be_bytes ()) . chain_update (& tmp) . finalize_into (& mut tmp) ; } hasher . update (& tmp) ; } VerifyingKey { id , q , k : hasher . finalize () , } } }
};
}
