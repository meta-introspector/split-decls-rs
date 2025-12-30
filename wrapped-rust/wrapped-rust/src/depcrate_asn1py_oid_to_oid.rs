// Generated macro for py_oid_to_oid (function)
macro_rules! Depcrate_asn1py_oid_to_oid {
() => {
// Module: crate::asn1
// Provides: {"py_oid_to_oid"}
// Dependencies: {}
pub (crate) fn py_oid_to_oid (py_oid : pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> pyo3 :: PyResult < asn1 :: ObjectIdentifier > { Ok (py_oid . cast :: < crate :: oid :: ObjectIdentifier > () ? . get () . oid . clone ()) }
};
}
