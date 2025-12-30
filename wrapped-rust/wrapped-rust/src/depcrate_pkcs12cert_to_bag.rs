// Generated macro for cert_to_bag (function)
macro_rules! Depcrate_pkcs12cert_to_bag {
() => {
// Module: crate::pkcs12
// Provides: {"cert_to_bag"}
// Dependencies: {}
fn cert_to_bag < 'a > (cert : & 'a Certificate , friendly_name : Option < & 'a [u8] > , local_key_id : Option < & 'a [u8] > , is_java_trusted_cert : bool ,) -> CryptographyResult < cryptography_x509 :: pkcs12 :: SafeBag < 'a > > { Ok (cryptography_x509 :: pkcs12 :: SafeBag { _bag_id : asn1 :: DefinedByMarker :: marker () , bag_value : asn1 :: Explicit :: new (cryptography_x509 :: pkcs12 :: BagValue :: CertBag (Box :: new (cryptography_x509 :: pkcs12 :: CertBag { _cert_id : asn1 :: DefinedByMarker :: marker () , cert_value : asn1 :: Explicit :: new (cryptography_x509 :: pkcs12 :: CertType :: X509 (asn1 :: OctetStringEncoded :: new (cert . raw . borrow_dependent () . clone ()) ,)) , } ,))) , attributes : pkcs12_attributes (friendly_name , local_key_id , is_java_trusted_cert) ? , }) }
};
}
