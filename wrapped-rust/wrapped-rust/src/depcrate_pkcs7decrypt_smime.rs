// Generated macro for decrypt_smime (function)
macro_rules! Depcrate_pkcs7decrypt_smime {
() => {
// Module: crate::pkcs7
// Provides: {"decrypt_smime"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn decrypt_smime < 'p > (py : pyo3 :: Python < 'p > , data : CffiBuf < 'p > , certificate : pyo3 :: Bound < 'p , x509 :: certificate :: Certificate > , private_key : pyo3 :: Bound < 'p , pyo3 :: types :: PyAny > , options : & pyo3 :: Bound < 'p , pyo3 :: types :: PyList > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { let decoded_smime_data = types :: SMIME_ENVELOPED_DECODE . get (py) ? . call1 ((data . as_bytes () ,)) ? ; let data = decoded_smime_data . extract () ? ; decrypt_der (py , data , certificate , private_key , options) }
};
}
