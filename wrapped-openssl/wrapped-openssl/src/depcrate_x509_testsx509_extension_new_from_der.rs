// Generated macro for x509_extension_new_from_der (function)
macro_rules! Depcrate_x509_testsx509_extension_new_from_der {
() => {
// Module: crate::x509::tests
// Provides: {"x509_extension_new_from_der"}
// Dependencies: {}
# [test] fn x509_extension_new_from_der () { let ext = X509Extension :: new_from_der (& Asn1Object :: from_str ("2.5.29.19") . unwrap () , true , & Asn1OctetString :: new_from_bytes (b"\x30\x03\x01\x01\xff") . unwrap () ,) . unwrap () ; assert_eq ! (ext . to_der () . unwrap () , b"0\x0f\x06\x03U\x1d\x13\x01\x01\xff\x04\x050\x03\x01\x01\xff") ; }
};
}
