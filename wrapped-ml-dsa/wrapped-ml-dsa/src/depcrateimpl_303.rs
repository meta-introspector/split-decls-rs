// Generated macro for impl_303 (impl)
macro_rules! Depcrateimpl_303 {
() => {
// Module: crate
// Provides: {"impl_303"}
// Dependencies: {}
impl < P : MlDsaParams > MultipartVerifier < Signature < P > > for VerifyingKey < P > { fn multipart_verify (& self , msg : & [& [u8]] , signature : & Signature < P >) -> Result < () , Error > { self . raw_verify_with_context (msg , & [] , signature) . then_some (()) . ok_or (Error :: new ()) } }
};
}
