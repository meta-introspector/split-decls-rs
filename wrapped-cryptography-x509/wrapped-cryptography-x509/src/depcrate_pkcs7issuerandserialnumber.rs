// Generated macro for IssuerAndSerialNumber (struct)
macro_rules! Depcrate_pkcs7IssuerAndSerialNumber {
() => {
// Module: crate::pkcs7
// Provides: {"IssuerAndSerialNumber"}
// Dependencies: {}
# [derive (asn1 :: Asn1Write , asn1 :: Asn1Read)] pub struct IssuerAndSerialNumber < 'a > { pub issuer : name :: Name < 'a > , pub serial_number : asn1 :: BigInt < 'a > , }
};
}
