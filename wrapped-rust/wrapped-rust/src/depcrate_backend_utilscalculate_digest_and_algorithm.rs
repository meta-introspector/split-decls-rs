// Generated macro for calculate_digest_and_algorithm (function)
macro_rules! Depcrate_backend_utilscalculate_digest_and_algorithm {
() => {
// Module: crate::backend::utils
// Provides: {"calculate_digest_and_algorithm"}
// Dependencies: {}
pub (crate) fn calculate_digest_and_algorithm < 'p > (py : pyo3 :: Python < 'p > , data : & 'p [u8] , algorithm : & pyo3 :: Bound < 'p , pyo3 :: PyAny > ,) -> CryptographyResult < (BytesOrPyBytes < 'p > , pyo3 :: Bound < 'p , pyo3 :: PyAny >) > { let (algorithm , data) = if algorithm . is_instance (& types :: PREHASHED . get (py) ?) ? { (algorithm . getattr ("_algorithm") ? , BytesOrPyBytes :: Bytes (data) ,) } else { let mut h = Hash :: new (py , algorithm , None) ? ; h . update_bytes (data) ? ; (algorithm . clone () , BytesOrPyBytes :: PyBytes (h . finalize (py) ?)) } ; if data . as_bytes () . len () != (algorithm . getattr ("digest_size") ? . extract :: < usize > () ?) { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("The provided data must be the same length as the hash algorithm's digest size." ,) ,)) ; } Ok ((data , algorithm)) }
};
}
