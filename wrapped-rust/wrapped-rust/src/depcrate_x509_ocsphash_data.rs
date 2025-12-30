// Generated macro for hash_data (function)
macro_rules! Depcrate_x509_ocsphash_data {
() => {
// Module: crate::x509::ocsp
// Provides: {"hash_data"}
// Dependencies: {}
pub (crate) fn hash_data < 'p > (py : pyo3 :: Python < 'p > , py_hash_alg : & pyo3 :: Bound < 'p , pyo3 :: PyAny > , data : & [u8] ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { let mut h = Hash :: new (py , py_hash_alg , None) ? ; h . update_bytes (data) ? ; Ok (h . finalize (py) ?) }
};
}
