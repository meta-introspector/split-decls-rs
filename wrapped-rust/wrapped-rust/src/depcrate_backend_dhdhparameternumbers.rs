// Generated macro for DHParameterNumbers (struct)
macro_rules! Depcrate_backend_dhDHParameterNumbers {
() => {
// Module: crate::backend::dh
// Provides: {"DHParameterNumbers"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.primitives.asymmetric.dh")] struct DHParameterNumbers { # [pyo3 (get)] p : pyo3 :: Py < pyo3 :: types :: PyInt > , # [pyo3 (get)] g : pyo3 :: Py < pyo3 :: types :: PyInt > , # [pyo3 (get)] q : Option < pyo3 :: Py < pyo3 :: types :: PyInt > > , }
};
}
