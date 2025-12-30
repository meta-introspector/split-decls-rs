// Generated macro for curve_supported (function)
macro_rules! Depcrate_backend_eccurve_supported {
() => {
// Module: crate::backend::ec
// Provides: {"curve_supported"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn curve_supported (py : pyo3 :: Python < '_ > , py_curve : pyo3 :: Bound < '_ , pyo3 :: PyAny >) -> bool { curve_from_py_curve (py , py_curve) . is_ok () }
};
}
