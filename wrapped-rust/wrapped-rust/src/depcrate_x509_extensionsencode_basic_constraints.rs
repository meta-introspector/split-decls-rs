// Generated macro for encode_basic_constraints (function)
macro_rules! Depcrate_x509_extensionsencode_basic_constraints {
() => {
// Module: crate::x509::extensions
// Provides: {"encode_basic_constraints"}
// Dependencies: {}
fn encode_basic_constraints (ext : & pyo3 :: Bound < '_ , pyo3 :: PyAny >) -> CryptographyResult < Vec < u8 > > { # [derive (pyo3 :: FromPyObject)] struct PyBasicConstraints { ca : bool , path_length : Option < u64 > , } let pybc = ext . extract :: < PyBasicConstraints > () ? ; let bc = extensions :: BasicConstraints { ca : pybc . ca , path_length : pybc . path_length , } ; Ok (asn1 :: write_single (& bc) ?) }
};
}
