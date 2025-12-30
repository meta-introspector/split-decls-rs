// Generated macro for impl_188 (impl)
macro_rules! Depcrate_ots_publicimpl_188 {
() => {
// Module: crate::ots::public
// Provides: {"impl_188"}
// Dependencies: {}
impl < Mode : LmsOtsMode > Verifier < Signature < Mode > > for VerifyingKey < Mode > where < Mode :: Hasher as OutputSizeUser > :: OutputSize : Add < U2 > , Sum < < Mode :: Hasher as OutputSizeUser > :: OutputSize , U2 > : ArraySize , { fn verify (& self , msg : & [u8] , signature : & Signature < Mode >) -> Result < () , Error > { self . multipart_verify (& [msg] , signature) } }
};
}
