// Generated macro for impl_189 (impl)
macro_rules! Depcrate_ots_publicimpl_189 {
() => {
// Module: crate::ots::public
// Provides: {"impl_189"}
// Dependencies: {}
impl < Mode : LmsOtsMode > MultipartVerifier < Signature < Mode > > for VerifyingKey < Mode > where < Mode :: Hasher as OutputSizeUser > :: OutputSize : Add < U2 > , Sum < < Mode :: Hasher as OutputSizeUser > :: OutputSize , U2 > : ArraySize , { fn multipart_verify (& self , msg : & [& [u8]] , signature : & Signature < Mode >) -> Result < () , Error > { let kc = signature . raw_recover_pubkey (self . id , self . q , msg) ; if self . k == kc . k { Ok (()) } else { Err (Error :: new ()) } } }
};
}
