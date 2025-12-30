// Generated macro for encode_oid_sequence (function)
macro_rules! Depcrate_x509_extensionsencode_oid_sequence {
() => {
// Module: crate::x509::extensions
// Provides: {"encode_oid_sequence"}
// Dependencies: {}
fn encode_oid_sequence (ext : & pyo3 :: Bound < '_ , pyo3 :: PyAny >) -> CryptographyResult < Vec < u8 > > { let mut oids = vec ! [] ; for el in ext . try_iter () ? { let oid = py_oid_to_oid (el ?) ? ; oids . push (oid) ; } Ok (asn1 :: write_single (& asn1 :: SequenceOfWriter :: new (oids)) ?) }
};
}
