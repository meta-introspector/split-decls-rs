// Generated macro for ML_KEM_1024 (const)
macro_rules! Depcrate_kemML_KEM_1024 {
() => {
// Module: crate::kem
// Provides: {"ML_KEM_1024"}
// Dependencies: {}
# [doc = " NIST FIPS 203 ML-KEM-1024 algorithm."] pub const ML_KEM_1024 : Algorithm < AlgorithmId > = Algorithm { id : AlgorithmId :: MlKem1024 , decapsulate_key_size : ML_KEM_1024_SECRET_KEY_LENGTH , encapsulate_key_size : ML_KEM_1024_PUBLIC_KEY_LENGTH , ciphertext_size : ML_KEM_1024_CIPHERTEXT_LENGTH , shared_secret_size : ML_KEM_1024_SHARED_SECRET_LENGTH , } ;
};
}
