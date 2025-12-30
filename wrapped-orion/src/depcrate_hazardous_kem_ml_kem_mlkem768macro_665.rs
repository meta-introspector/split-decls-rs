// Generated macro for macro_665 (macro)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem768macro_665 {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem768
// Provides: {"macro_665"}
// Dependencies: {}
construct_public ! { # [doc = " A type to represent the KEM `Ciphertext` that ML-KEM-768 returns."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 1088 bytes."] (Ciphertext , test_kem_ciphertext , MlKem768Internal :: CIPHERTEXT_SIZE , MlKem768Internal :: CIPHERTEXT_SIZE) }
};
}
