// Generated macro for Cert (struct)
macro_rules! Depcrate_certCert {
() => {
// Module: crate::cert
// Provides: {"Cert"}
// Dependencies: {}
# [doc = " A certificate for a remote connection, viewable as one of `CertHostkey` or"] # [doc = " `CertX509` currently."] pub struct Cert < 'a > { raw : * mut raw :: git_cert , _marker : marker :: PhantomData < & 'a raw :: git_cert > , }
};
}
