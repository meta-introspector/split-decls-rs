macro_rules! deps {
    () => {
        MlKem768Internal!();
    };
}

macro_rules! macro_503 {
    () => {
        deps!();
        construct_public ! { # [doc = " A type to represent the KEM `Ciphertext` that ML-KEM-768 returns."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 1088 bytes."] (Ciphertext , test_kem_ciphertext , MlKem768Internal :: CIPHERTEXT_SIZE , MlKem768Internal :: CIPHERTEXT_SIZE) }
    };
}

macro_503!();