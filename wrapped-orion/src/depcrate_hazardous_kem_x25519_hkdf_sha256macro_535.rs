// Generated macro for macro_535 (macro)
macro_rules! Depcrate_hazardous_kem_x25519_hkdf_sha256macro_535 {
() => {
// Module: crate::hazardous::kem::x25519_hkdf_sha256
// Provides: {"macro_535"}
// Dependencies: {}
construct_secret_key ! { # [doc = " A type to represent the `SharedSecret` that DH-KEM(X25519, HKDF-SHA256) produces."] # [doc = ""] # [doc = " This type simply holds bytes. Creating an instance from slices or similar,"] # [doc = " performs no checks whatsoever."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 32 bytes."] (SharedSecret , test_shared_key , 32 , 32) }
};
}
