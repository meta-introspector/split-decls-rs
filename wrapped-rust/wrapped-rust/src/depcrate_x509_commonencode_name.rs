// Generated macro for encode_name (function)
macro_rules! Depcrate_x509_commonencode_name {
() => {
// Module: crate::x509::common
// Provides: {"encode_name"}
// Dependencies: {}
pub (crate) fn encode_name < 'p > (py : pyo3 :: Python < '_ > , ka : & 'p cryptography_keepalive :: KeepAlive < pyo3 :: pybacked :: PyBackedBytes > , py_name : & pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> pyo3 :: PyResult < Name < 'p > > { let mut rdns = vec ! [] ; for py_rdn in py_name . getattr (pyo3 :: intern ! (py , "rdns")) ? . try_iter () ? { let py_rdn = py_rdn ? ; let mut attrs = vec ! [] ; for py_attr in py_rdn . try_iter () ? { attrs . push (encode_name_entry (py , ka , & py_attr ?) ?) ; } rdns . push (asn1 :: SetOfWriter :: new (attrs)) ; } Ok (Asn1ReadableOrWritable :: new_write (asn1 :: SequenceOfWriter :: new (rdns) ,)) }
};
}
