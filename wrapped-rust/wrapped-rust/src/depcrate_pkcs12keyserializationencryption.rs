// Generated macro for KeySerializationEncryption (struct)
macro_rules! Depcrate_pkcs12KeySerializationEncryption {
() => {
// Module: crate::pkcs12
// Provides: {"KeySerializationEncryption"}
// Dependencies: {}
struct KeySerializationEncryption < 'a > { password : pyo3 :: pybacked :: PyBackedBytes , mac_algorithm : pyo3 :: Bound < 'a , pyo3 :: PyAny > , mac_kdf_iter : u64 , cipher_kdf_iter : u64 , encryption_algorithm : Option < pbe :: EncryptionAlgorithm > , }
};
}
