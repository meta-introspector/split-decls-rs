// Generated macro for macro_704 (macro)
macro_rules! Depcrate_hazardous_kem_ml_kemmacro_704 {
() => {
// Module: crate::hazardous::kem::ml_kem
// Provides: {"macro_704"}
// Dependencies: {}
construct_secret_key ! { # [doc = " A type to represent the `d||z` seed used by ML-KEM to produce"] # [doc = " a decapsulation key and its corresponding encapsulation key."] # [doc = ""] # [doc = " It it crucial for the security of ML-KEM that these be generated"] # [doc = " using a CSPRNG."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 64 bytes."] (Seed , test_ml_kem_seed , 64 , 64 , 64) }
};
}
