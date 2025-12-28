macro_rules! deps {
    () => {
        MlKem1024Internal!();
    };
}

macro_rules! macro_522 {
    () => {
        deps!();
        construct_public ! { # [doc = " A type to represent the KEM `Ciphertext` that ML-KEM-1024 returns."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 1568 bytes."] (Ciphertext , test_kem_ciphertext , MlKem1024Internal :: CIPHERTEXT_SIZE , MlKem1024Internal :: CIPHERTEXT_SIZE) }
    };
}

macro_522!();