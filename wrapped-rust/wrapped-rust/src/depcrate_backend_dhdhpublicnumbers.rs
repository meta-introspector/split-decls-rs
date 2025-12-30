// Generated macro for DHPublicNumbers (struct)
macro_rules! Depcrate_backend_dhDHPublicNumbers {
() => {
// Module: crate::backend::dh
// Provides: {"DHPublicNumbers"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.primitives.asymmetric.dh")] struct DHPublicNumbers { # [pyo3 (get)] y : pyo3 :: Py < pyo3 :: types :: PyInt > , # [pyo3 (get)] parameter_numbers : pyo3 :: Py < DHParameterNumbers > , }
};
}
