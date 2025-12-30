// Generated macro for py_curve_from_curve (function)
macro_rules! Depcrate_backend_ecpy_curve_from_curve {
() => {
// Module: crate::backend::ec
// Provides: {"py_curve_from_curve"}
// Dependencies: {}
fn py_curve_from_curve < 'p > (py : pyo3 :: Python < 'p > , curve : & openssl :: ec :: EcGroupRef ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { assert ! (curve . asn1_flag () != openssl :: ec :: Asn1Flag :: EXPLICIT_CURVE) ; let name = curve . curve_name () . unwrap () . short_name () ? ; Ok (types :: CURVE_TYPES . get (py) ? . get_item (name) ?) }
};
}
