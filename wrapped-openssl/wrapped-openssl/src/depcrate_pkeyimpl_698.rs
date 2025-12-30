// Generated macro for impl_698 (impl)
macro_rules! Depcrate_pkeyimpl_698 {
() => {
// Module: crate::pkey
// Provides: {"impl_698"}
// Dependencies: {}
impl Id { pub const RSA : Id = Id (ffi :: EVP_PKEY_RSA) ; # [cfg (any (ossl111 , libressl , boringssl , awslc))] pub const RSA_PSS : Id = Id (ffi :: EVP_PKEY_RSA_PSS) ; # [cfg (not (boringssl))] pub const HMAC : Id = Id (ffi :: EVP_PKEY_HMAC) ; # [cfg (not (any (boringssl , awslc)))] pub const CMAC : Id = Id (ffi :: EVP_PKEY_CMAC) ; pub const DSA : Id = Id (ffi :: EVP_PKEY_DSA) ; pub const DH : Id = Id (ffi :: EVP_PKEY_DH) ; # [cfg (ossl110)] pub const DHX : Id = Id (ffi :: EVP_PKEY_DHX) ; pub const EC : Id = Id (ffi :: EVP_PKEY_EC) ; # [cfg (ossl111)] pub const SM2 : Id = Id (ffi :: EVP_PKEY_SM2) ; # [cfg (any (ossl110 , boringssl , libressl360 , awslc))] pub const HKDF : Id = Id (ffi :: EVP_PKEY_HKDF) ; # [cfg (any (ossl111 , boringssl , libressl370 , awslc))] pub const ED25519 : Id = Id (ffi :: EVP_PKEY_ED25519) ; # [cfg (ossl111)] pub const ED448 : Id = Id (ffi :: EVP_PKEY_ED448) ; # [cfg (any (ossl111 , boringssl , libressl370 , awslc))] pub const X25519 : Id = Id (ffi :: EVP_PKEY_X25519) ; # [cfg (ossl111)] pub const X448 : Id = Id (ffi :: EVP_PKEY_X448) ; # [cfg (ossl111)] pub const POLY1305 : Id = Id (ffi :: EVP_PKEY_POLY1305) ; # [doc = " Creates a `Id` from an integer representation."] pub fn from_raw (value : c_int) -> Id { Id (value) } # [doc = " Returns the integer representation of the `Id`."] # [allow (clippy :: trivially_copy_pass_by_ref)] pub fn as_raw (& self) -> c_int { self . 0 } }
};
}
