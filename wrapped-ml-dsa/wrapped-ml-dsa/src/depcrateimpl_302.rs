// Generated macro for impl_302 (impl)
macro_rules! Depcrateimpl_302 {
() => {
// Module: crate
// Provides: {"impl_302"}
// Dependencies: {}
impl < P : MlDsaParams > signature :: Verifier < Signature < P > > for VerifyingKey < P > { fn verify (& self , msg : & [u8] , signature : & Signature < P >) -> Result < () , Error > { self . multipart_verify (& [msg] , signature) } }
};
}
