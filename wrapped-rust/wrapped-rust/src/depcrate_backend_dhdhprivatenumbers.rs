// Generated macro for DHPrivateNumbers (struct)
macro_rules! Depcrate_backend_dhDHPrivateNumbers {
() => {
// Module: crate::backend::dh
// Provides: {"DHPrivateNumbers"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.primitives.asymmetric.dh")] struct DHPrivateNumbers { # [pyo3 (get)] x : pyo3 :: Py < pyo3 :: types :: PyInt > , # [pyo3 (get)] public_numbers : pyo3 :: Py < DHPublicNumbers > , }
};
}
