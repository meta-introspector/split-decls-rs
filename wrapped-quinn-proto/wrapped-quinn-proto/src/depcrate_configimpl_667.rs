// Generated macro for impl_667 (impl)
macro_rules! Depcrate_configimpl_667 {
() => {
// Module: crate::config
// Provides: {"impl_667"}
// Dependencies: {}
# [cfg (any (feature = "rustls-aws-lc-rs" , feature = "rustls-ring"))] impl ClientConfig { # [doc = " Create a client configuration that trusts the platform's native roots"] # [deprecated (since = "0.11.13" , note = "use `try_with_platform_verifier()` instead")] # [cfg (feature = "platform-verifier")] pub fn with_platform_verifier () -> Self { Self :: try_with_platform_verifier () . expect ("use try_with_platform_verifier() instead") } # [doc = " Create a client configuration that trusts the platform's native roots"] # [cfg (feature = "platform-verifier")] pub fn try_with_platform_verifier () -> Result < Self , rustls :: Error > { Ok (Self :: new (Arc :: new (crypto :: rustls :: QuicClientConfig :: with_platform_verifier () ? ,))) } # [doc = " Create a client configuration that trusts specified trust anchors"] pub fn with_root_certificates (roots : Arc < rustls :: RootCertStore > ,) -> Result < Self , rustls :: client :: VerifierBuilderError > { Ok (Self :: new (Arc :: new (crypto :: rustls :: QuicClientConfig :: new (WebPkiServerVerifier :: builder_with_provider (roots , configured_provider ()) . build () ? ,)))) } }
};
}
