// Generated macro for big_cert_and_key (function)
macro_rules! Depcrate_testsbig_cert_and_key {
() => {
// Module: crate::tests
// Provides: {"big_cert_and_key"}
// Dependencies: {}
# [doc = " Generate a big fat certificate that can't fit inside the initial anti-amplification limit"] fn big_cert_and_key () -> (CertificateDer < 'static > , PrivateKeyDer < 'static >) { let cert = rcgen :: generate_simple_self_signed (Some ("localhost" . into ()) . into_iter () . chain ((0 .. 1000) . map (| x | format ! ("foo_{x}"))) . collect :: < Vec < _ > > () ,) . unwrap () ; (cert . cert . into () , PrivateKeyDer :: Pkcs8 (cert . signing_key . serialize_der () . into ()) ,) }
};
}
