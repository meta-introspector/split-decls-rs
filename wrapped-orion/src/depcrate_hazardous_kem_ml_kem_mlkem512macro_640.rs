// Generated macro for macro_640 (macro)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem512macro_640 {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem512
// Provides: {"macro_640"}
// Dependencies: {}
construct_secret_key ! { # [doc = " A type to represent the `SharedSecret` that ML-KEM-512 produces."] # [doc = ""] # [doc = " This type simply holds bytes. Creating an instance from slices or similar,"] # [doc = " performs no checks whatsoever."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 32 bytes."] (SharedSecret , test_shared_key , MlKem512Internal :: SHARED_SECRET_SIZE , MlKem512Internal :: SHARED_SECRET_SIZE) }
};
}
