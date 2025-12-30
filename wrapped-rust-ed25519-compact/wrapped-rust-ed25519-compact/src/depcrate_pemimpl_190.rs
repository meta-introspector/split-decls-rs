// Generated macro for impl_190 (impl)
macro_rules! Depcrate_pemimpl_190 {
() => {
// Module: crate::pem
// Provides: {"impl_190"}
// Dependencies: {}
impl SecretKey { # [doc = " Import a secret key from an OpenSSL-compatible DER file."] pub fn from_der (der : & [u8]) -> Result < Self , Error > { let kp = KeyPair :: from_der (der) ? ; Ok (kp . sk) } # [doc = " Import a secret key from an OpenSSL-compatible PEM file."] pub fn from_pem (pem : & str) -> Result < Self , Error > { let kp = KeyPair :: from_pem (pem) ? ; Ok (kp . sk) } # [doc = " Export a secret key as an OpenSSL-compatible DER file."] # [cfg (feature = "std")] pub fn to_der (& self) -> Vec < u8 > { let mut der = [0u8 ; 16 + Seed :: BYTES] ; der [0 .. 16] . copy_from_slice (& DER_HEADER_SK) ; der [16 ..] . copy_from_slice (self . seed () . as_ref ()) ; der . to_vec () } # [doc = " Export a secret key as an OpenSSL-compatible PEM file."] # [cfg (feature = "std")] pub fn to_pem (& self) -> String { let b64 = Base64 :: encode_to_string (self . to_der ()) . unwrap () ; format ! ("-----BEGIN PRIVATE KEY-----\n{}\n-----END PRIVATE KEY-----\n" , b64) } }
};
}
