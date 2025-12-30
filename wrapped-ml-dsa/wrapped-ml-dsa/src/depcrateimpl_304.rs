// Generated macro for impl_304 (impl)
macro_rules! Depcrateimpl_304 {
() => {
// Module: crate
// Provides: {"impl_304"}
// Dependencies: {}
impl < P : MlDsaParams > DigestVerifier < Shake256 , Signature < P > > for VerifyingKey < P > { fn verify_digest < F : Fn (& mut Shake256) -> Result < () , Error > > (& self , f : F , signature : & Signature < P > ,) -> Result < () , Error > { let mut mu = MuBuilder :: new (& self . tr , & []) ; f (mu . as_mut ()) ? ; let mu = mu . finish () ; self . raw_verify_mu (& mu , signature) . then_some (()) . ok_or (Error :: new ()) } }
};
}
