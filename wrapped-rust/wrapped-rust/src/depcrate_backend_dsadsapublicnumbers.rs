// Generated macro for DsaPublicNumbers (struct)
macro_rules! Depcrate_backend_dsaDsaPublicNumbers {
() => {
// Module: crate::backend::dsa
// Provides: {"DsaPublicNumbers"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.primitives.asymmetric.dsa" , name = "DSAPublicNumbers")] struct DsaPublicNumbers { # [pyo3 (get)] y : pyo3 :: Py < pyo3 :: types :: PyInt > , # [pyo3 (get)] parameter_numbers : pyo3 :: Py < DsaParameterNumbers > , }
};
}
