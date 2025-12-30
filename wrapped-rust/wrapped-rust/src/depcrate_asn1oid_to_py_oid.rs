// Generated macro for oid_to_py_oid (function)
macro_rules! Depcrate_asn1oid_to_py_oid {
() => {
// Module: crate::asn1
// Provides: {"oid_to_py_oid"}
// Dependencies: {}
pub (crate) fn oid_to_py_oid < 'p > (py : pyo3 :: Python < 'p > , oid : & asn1 :: ObjectIdentifier ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { Ok (pyo3 :: Bound :: new (py , crate :: oid :: ObjectIdentifier { oid : oid . clone () }) ? . into_any ()) }
};
}
