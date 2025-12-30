// Generated macro for ML_KEM_512 (const)
macro_rules! Depcrate_kemML_KEM_512 {
() => {
// Module: crate::kem
// Provides: {"ML_KEM_512"}
// Dependencies: {}
# [doc = " NIST FIPS 203 ML-KEM-512 algorithm."] pub const ML_KEM_512 : Algorithm < AlgorithmId > = Algorithm { id : AlgorithmId :: MlKem512 , decapsulate_key_size : ML_KEM_512_SECRET_KEY_LENGTH , encapsulate_key_size : ML_KEM_512_PUBLIC_KEY_LENGTH , ciphertext_size : ML_KEM_512_CIPHERTEXT_LENGTH , shared_secret_size : ML_KEM_512_SHARED_SECRET_LENGTH , } ;
};
}
