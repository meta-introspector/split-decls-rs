// Generated macro for big_byte_slice_to_py_int (function)
macro_rules! Depcrate_asn1big_byte_slice_to_py_int {
() => {
// Module: crate::asn1
// Provides: {"big_byte_slice_to_py_int"}
// Dependencies: {}
pub (crate) fn big_byte_slice_to_py_int < 'p > (py : pyo3 :: Python < 'p > , v : & '_ [u8] ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let int_type = py . get_type :: < pyo3 :: types :: PyInt > () ; let kwargs = [("signed" , true)] . into_py_dict (py) ? ; int_type . call_method (pyo3 :: intern ! (py , "from_bytes") , (v , "big") , Some (& kwargs)) }
};
}
