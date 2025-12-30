// Generated macro for ML_KEM_768 (const)
macro_rules! Depcrate_kemML_KEM_768 {
() => {
// Module: crate::kem
// Provides: {"ML_KEM_768"}
// Dependencies: {}
# [doc = " NIST FIPS 203 ML-KEM-768 algorithm."] pub const ML_KEM_768 : Algorithm < AlgorithmId > = Algorithm { id : AlgorithmId :: MlKem768 , decapsulate_key_size : ML_KEM_768_SECRET_KEY_LENGTH , encapsulate_key_size : ML_KEM_768_PUBLIC_KEY_LENGTH , ciphertext_size : ML_KEM_768_CIPHERTEXT_LENGTH , shared_secret_size : ML_KEM_768_SHARED_SECRET_LENGTH , } ;
};
}
