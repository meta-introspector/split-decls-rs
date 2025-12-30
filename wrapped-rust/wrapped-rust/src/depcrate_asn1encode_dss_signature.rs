// Generated macro for encode_dss_signature (function)
macro_rules! Depcrate_asn1encode_dss_signature {
() => {
// Module: crate::asn1
// Provides: {"encode_dss_signature"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn encode_dss_signature < 'p > (py : pyo3 :: Python < 'p > , r : pyo3 :: Bound < '_ , pyo3 :: types :: PyInt > , s : pyo3 :: Bound < '_ , pyo3 :: types :: PyInt > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { let r_bytes = py_uint_to_big_endian_bytes (py , r) ? ; let s_bytes = py_uint_to_big_endian_bytes (py , s) ? ; let sig = DssSignature { r : asn1 :: BigUint :: new (& r_bytes) . unwrap () , s : asn1 :: BigUint :: new (& s_bytes) . unwrap () , } ; let result = asn1 :: write_single (& sig) ? ; Ok (pyo3 :: types :: PyBytes :: new (py , & result)) }
};
}
