// Generated macro for macro_725 (macro)
macro_rules! Depcrate_hazardous_kem_xwingmacro_725 {
() => {
// Module: crate::hazardous::kem::xwing
// Provides: {"macro_725"}
// Dependencies: {}
construct_secret_key ! { # [doc = " A type to represent the private `Seed` that X-Wing uses."] # [doc = ""] # [doc = " This type simply holds bytes. Creating an instance from slices or similar,"] # [doc = " performs no checks whatsoever."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 32 bytes."] (Seed , test_seed , PRIVATE_KEY_SIZE , PRIVATE_KEY_SIZE , PRIVATE_KEY_SIZE) }
};
}
