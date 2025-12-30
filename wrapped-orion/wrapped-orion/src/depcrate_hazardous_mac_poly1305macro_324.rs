// Generated macro for macro_324 (macro)
macro_rules! Depcrate_hazardous_mac_poly1305macro_324 {
() => {
// Module: crate::hazardous::mac::poly1305
// Provides: {"macro_324"}
// Dependencies: {}
construct_secret_key ! { # [doc = " A type to represent the `OneTimeKey` that Poly1305 uses for authentication."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 32 bytes."] # [doc = ""] # [doc = " # Panics:"] # [doc = " A panic will occur if:"] # [doc = " - Failure to generate random bytes securely."] (OneTimeKey , test_one_time_key , POLY1305_KEYSIZE , POLY1305_KEYSIZE , POLY1305_KEYSIZE) }
};
}
