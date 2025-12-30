// Generated macro for encode_extensions (function)
macro_rules! Depcrate_x509_commonencode_extensions {
() => {
// Module: crate::x509::common
// Provides: {"encode_extensions"}
// Dependencies: {}
pub (crate) fn encode_extensions < 'p , F : Fn (pyo3 :: Python < '_ > , & asn1 :: ObjectIdentifier , & pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> CryptographyResult < Option < Vec < u8 > > > , > (py : pyo3 :: Python < 'p > , ka_vec : & 'p cryptography_keepalive :: KeepAlive < Vec < u8 > > , ka_bytes : & 'p cryptography_keepalive :: KeepAlive < pyo3 :: pybacked :: PyBackedBytes > , py_exts : & pyo3 :: Bound < 'p , pyo3 :: PyAny > , encode_ext : F ,) -> pyo3 :: PyResult < Option < RawExtensions < 'p > > > { let mut exts = vec ! [] ; for py_ext in py_exts . try_iter () ? { let py_ext = py_ext ? ; let py_oid = py_ext . getattr (pyo3 :: intern ! (py , "oid")) ? ; let oid = py_oid_to_oid (py_oid) ? ; let ext_val = py_ext . getattr (pyo3 :: intern ! (py , "value")) ? ; if ext_val . is_instance (& types :: UNRECOGNIZED_EXTENSION . get (py) ?) ? { exts . push (Extension { extn_id : oid , critical : py_ext . getattr (pyo3 :: intern ! (py , "critical")) ? . extract () ? , extn_value : ka_bytes . add (ext_val . getattr (pyo3 :: intern ! (py , "value")) ? . extract () ?) , }) ; continue ; } match encode_ext (py , & oid , & ext_val) ? { Some (data) => { exts . push (Extension { extn_id : oid , critical : py_ext . getattr (pyo3 :: intern ! (py , "critical")) ? . extract () ? , extn_value : ka_vec . add (data) , }) ; } None => { return Err (pyo3 :: exceptions :: PyNotImplementedError :: new_err (format ! ("Extension not supported: {oid}"))) } } } if exts . is_empty () { return Ok (None) ; } Ok (Some (Asn1ReadableOrWritable :: new_write (asn1 :: SequenceOfWriter :: new (exts) ,))) }
};
}
