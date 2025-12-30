// Generated macro for DsaPrivateNumbers (struct)
macro_rules! Depcrate_backend_dsaDsaPrivateNumbers {
() => {
// Module: crate::backend::dsa
// Provides: {"DsaPrivateNumbers"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.primitives.asymmetric.dsa" , name = "DSAPrivateNumbers")] struct DsaPrivateNumbers { # [pyo3 (get)] x : pyo3 :: Py < pyo3 :: types :: PyInt > , # [pyo3 (get)] public_numbers : pyo3 :: Py < DsaPublicNumbers > , }
};
}
