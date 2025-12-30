// Generated macro for py_uint_to_be_bytes_with_length (function)
macro_rules! Depcrate_asn1py_uint_to_be_bytes_with_length {
() => {
// Module: crate::asn1
// Provides: {"py_uint_to_be_bytes_with_length"}
// Dependencies: {}
pub (crate) fn py_uint_to_be_bytes_with_length < 'p > (py : pyo3 :: Python < 'p > , v : pyo3 :: Bound < 'p , pyo3 :: types :: PyInt > , length : usize ,) -> pyo3 :: PyResult < PyBackedBytes > { if v . lt (0) ? { return Err (pyo3 :: exceptions :: PyValueError :: new_err ("Negative integers are not supported" ,)) ; } Ok (v . call_method1 (pyo3 :: intern ! (py , "to_bytes") , (length , "big")) ? . extract () ? ,) }
};
}
