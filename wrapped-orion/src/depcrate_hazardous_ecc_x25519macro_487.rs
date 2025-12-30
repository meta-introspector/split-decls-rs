// Generated macro for macro_487 (macro)
macro_rules! Depcrate_hazardous_ecc_x25519macro_487 {
() => {
// Module: crate::hazardous::ecc::x25519
// Provides: {"macro_487"}
// Dependencies: {}
construct_secret_key ! { # [doc = " A type to represent the `SharedKey` that X25519 produces."] # [doc = ""] # [doc = " This type simply holds bytes. Creating an instance from slices or similar,"] # [doc = " performs no checks whatsoever."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 32 bytes."] (SharedKey , test_shared_key , SHARED_KEY_SIZE , SHARED_KEY_SIZE) }
};
}
