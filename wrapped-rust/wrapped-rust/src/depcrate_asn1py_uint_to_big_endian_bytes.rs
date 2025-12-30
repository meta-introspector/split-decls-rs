// Generated macro for py_uint_to_big_endian_bytes (function)
macro_rules! Depcrate_asn1py_uint_to_big_endian_bytes {
() => {
// Module: crate::asn1
// Provides: {"py_uint_to_big_endian_bytes"}
// Dependencies: {}
pub (crate) fn py_uint_to_big_endian_bytes < 'p > (py : pyo3 :: Python < 'p > , v : pyo3 :: Bound < 'p , pyo3 :: types :: PyInt > ,) -> pyo3 :: PyResult < PyBackedBytes > { let length = v . call_method0 (pyo3 :: intern ! (py , "bit_length")) ? . extract :: < usize > () ? / 8 + 1 ; py_uint_to_be_bytes_with_length (py , v , length) }
};
}
