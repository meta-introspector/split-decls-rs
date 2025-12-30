// Generated macro for macro_686 (macro)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem1024macro_686 {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem1024
// Provides: {"macro_686"}
// Dependencies: {}
construct_secret_key ! { # [doc = " A type to represent the `SharedSecret` that ML-KEM-1024 produces."] # [doc = ""] # [doc = " This type simply holds bytes. Creating an instance from slices or similar,"] # [doc = " performs no checks whatsoever."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 32 bytes."] (SharedSecret , test_shared_key , MlKem1024Internal :: SHARED_SECRET_SIZE , MlKem1024Internal :: SHARED_SECRET_SIZE) }
};
}
