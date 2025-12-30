// Generated macro for impl_208 (impl)
macro_rules! Depcrate_certimpl_208 {
() => {
// Module: crate::cert
// Provides: {"impl_208"}
// Dependencies: {}
impl < 'a > Cert < 'a > { # [doc = " Attempt to view this certificate as an SSH hostkey."] # [doc = ""] # [doc = " Returns `None` if this is not actually an SSH hostkey."] pub fn as_hostkey (& self) -> Option < & CertHostkey < 'a > > { self . cast (raw :: GIT_CERT_HOSTKEY_LIBSSH2) } # [doc = " Attempt to view this certificate as an X.509 certificate."] # [doc = ""] # [doc = " Returns `None` if this is not actually an X.509 certificate."] pub fn as_x509 (& self) -> Option < & CertX509 < 'a > > { self . cast (raw :: GIT_CERT_X509) } fn cast < T > (& self , kind : raw :: git_cert_t) -> Option < & T > { assert_eq ! (mem :: size_of ::< Cert <'a >> () , mem :: size_of ::< T > ()) ; unsafe { if kind == (* self . raw) . cert_type { Some (& * (self as * const Cert < 'a > as * const T)) } else { None } } } }
};
}
