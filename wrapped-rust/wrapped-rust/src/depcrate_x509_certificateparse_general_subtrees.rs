// Generated macro for parse_general_subtrees (function)
macro_rules! Depcrate_x509_certificateparse_general_subtrees {
() => {
// Module: crate::x509::certificate
// Provides: {"parse_general_subtrees"}
// Dependencies: {}
fn parse_general_subtrees < 'p > (py : pyo3 :: Python < 'p > , subtrees : SequenceOfSubtrees < '_ , Asn1Read > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let gns = pyo3 :: types :: PyList :: empty (py) ; for gs in subtrees { gns . append (x509 :: parse_general_name (py , gs . base) ?) ? ; } Ok (gns . into_any ()) }
};
}
