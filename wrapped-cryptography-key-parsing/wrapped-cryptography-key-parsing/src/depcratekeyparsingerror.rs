// Generated macro for KeyParsingError (enum)
macro_rules! DepcrateKeyParsingError {
() => {
// Module: crate
// Provides: {"KeyParsingError"}
// Dependencies: {}
pub enum KeyParsingError { InvalidKey , ExplicitCurveUnsupported , UnsupportedKeyType (asn1 :: ObjectIdentifier) , UnsupportedEllipticCurve (asn1 :: ObjectIdentifier) , Parse (asn1 :: ParseError) , OpenSSL (openssl :: error :: ErrorStack) , UnsupportedEncryptionAlgorithm (asn1 :: ObjectIdentifier) , EncryptedKeyWithoutPassword , IncorrectPassword , TruncatedEcPrivateKey , PemMissingDekInfo , PemInvalidDekInfo , PemInvalidIv , PemUnableToDeriveKey , PemUnsupportedCipher , PemInvalidProcType , }
};
}
