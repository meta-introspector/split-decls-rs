macro_rules! deps {
    () => {
        MlKem512Internal!();
    };
}

macro_rules! macro_484 {
    () => {
        deps!();
        construct_public ! { # [doc = " A type to represent the KEM `Ciphertext` that ML-KEM-512 returns."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 768 bytes."] (Ciphertext , test_kem_ciphertext , MlKem512Internal :: CIPHERTEXT_SIZE , MlKem512Internal :: CIPHERTEXT_SIZE) }
    };
}

macro_484!();