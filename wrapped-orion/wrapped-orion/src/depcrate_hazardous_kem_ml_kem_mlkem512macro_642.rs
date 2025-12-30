// Generated macro for macro_642 (macro)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem512macro_642 {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem512
// Provides: {"macro_642"}
// Dependencies: {}
construct_public ! { # [doc = " A type to represent the KEM `Ciphertext` that ML-KEM-512 returns."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 768 bytes."] (Ciphertext , test_kem_ciphertext , MlKem512Internal :: CIPHERTEXT_SIZE , MlKem512Internal :: CIPHERTEXT_SIZE) }
};
}
