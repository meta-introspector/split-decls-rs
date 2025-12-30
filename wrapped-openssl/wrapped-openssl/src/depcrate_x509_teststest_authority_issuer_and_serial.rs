// Generated macro for test_authority_issuer_and_serial (function)
macro_rules! Depcrate_x509_teststest_authority_issuer_and_serial {
() => {
// Module: crate::x509::tests
// Provides: {"test_authority_issuer_and_serial"}
// Dependencies: {}
# [test] # [cfg (ossl111d)] fn test_authority_issuer_and_serial () { let cert = include_bytes ! ("../../test/authority_key_identifier.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let authority_issuer = cert . authority_issuer () . unwrap () ; assert_eq ! (1 , authority_issuer . len ()) ; let dn = authority_issuer [0] . directory_name () . unwrap () ; let mut o = dn . entries_by_nid (Nid :: ORGANIZATIONNAME) ; let o = o . next () . unwrap () . data () . as_utf8 () . unwrap () ; assert_eq ! (o . as_bytes () , b"PyCA") ; let mut cn = dn . entries_by_nid (Nid :: COMMONNAME) ; let cn = cn . next () . unwrap () . data () . as_utf8 () . unwrap () ; assert_eq ! (cn . as_bytes () , b"cryptography.io") ; let authority_serial = cert . authority_serial () . unwrap () ; let serial = authority_serial . to_bn () . unwrap () ; let expected = BigNum :: from_u32 (3) . unwrap () ; assert_eq ! (serial , expected) ; }
};
}
