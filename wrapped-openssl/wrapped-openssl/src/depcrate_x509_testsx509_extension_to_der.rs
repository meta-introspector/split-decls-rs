// Generated macro for x509_extension_to_der (function)
macro_rules! Depcrate_x509_testsx509_extension_to_der {
() => {
// Module: crate::x509::tests
// Provides: {"x509_extension_to_der"}
// Dependencies: {}
# [test] fn x509_extension_to_der () { let builder = X509 :: builder () . unwrap () ; for (ext , expected) in [(BasicConstraints :: new () . critical () . ca () . build () . unwrap () , b"0\x0f\x06\x03U\x1d\x13\x01\x01\xff\x04\x050\x03\x01\x01\xff" as & [u8] ,) , (SubjectAlternativeName :: new () . dns ("example.com,DNS:example2.com") . build (& builder . x509v3_context (None , None)) . unwrap () , b"0'\x06\x03U\x1d\x11\x04 0\x1e\x82\x1cexample.com,DNS:example2.com" ,) , (SubjectAlternativeName :: new () . rid ("1.2.3.4") . uri ("https://example.com") . build (& builder . x509v3_context (None , None)) . unwrap () , b"0#\x06\x03U\x1d\x11\x04\x1c0\x1a\x88\x03*\x03\x04\x86\x13https://example.com" ,) , (ExtendedKeyUsage :: new () . server_auth () . other ("2.999.1") . other ("clientAuth") . build () . unwrap () , b"0\x22\x06\x03U\x1d%\x04\x1b0\x19\x06\x08+\x06\x01\x05\x05\x07\x03\x01\x06\x03\x887\x01\x06\x08+\x06\x01\x05\x05\x07\x03\x02" ,) ,] { assert_eq ! (& ext . to_der () . unwrap () , expected) ; } }
};
}
