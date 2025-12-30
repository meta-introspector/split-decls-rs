// Generated macro for decode_dss_signature (function)
macro_rules! Depcrate_asn1decode_dss_signature {
() => {
// Module: crate::asn1
// Provides: {"decode_dss_signature"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn decode_dss_signature < 'p > (py : pyo3 :: Python < 'p > , data : & [u8] ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let sig = asn1 :: parse_single :: < DssSignature < '_ > > (data) ? ; Ok ((big_byte_slice_to_py_int (py , sig . r . as_bytes ()) ? , big_byte_slice_to_py_int (py , sig . s . as_bytes ()) ? ,) . into_pyobject (py) ? . into_any ()) }
};
}
