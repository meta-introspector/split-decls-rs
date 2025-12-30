// Generated macro for lookup_scheme (function)
macro_rules! Depcratelookup_scheme {
() => {
// Module: crate
// Provides: {"lookup_scheme"}
// Dependencies: {}
fn lookup_scheme (scheme : u16) -> SignatureScheme { match scheme { 0x0401 => SignatureScheme :: RSA_PKCS1_SHA256 , 0x0501 => SignatureScheme :: RSA_PKCS1_SHA384 , 0x0601 => SignatureScheme :: RSA_PKCS1_SHA512 , 0x0403 => SignatureScheme :: ECDSA_NISTP256_SHA256 , 0x0503 => SignatureScheme :: ECDSA_NISTP384_SHA384 , 0x0603 => SignatureScheme :: ECDSA_NISTP521_SHA512 , 0x0804 => SignatureScheme :: RSA_PSS_SHA256 , 0x0805 => SignatureScheme :: RSA_PSS_SHA384 , 0x0806 => SignatureScheme :: RSA_PSS_SHA512 , 0x0807 => SignatureScheme :: ED25519 , _ => { println_err ! ("Unsupported signature scheme {:04x}" , scheme) ; process :: exit (BOGO_NACK) ; } } }
};
}
