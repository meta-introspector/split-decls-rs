// Generated macro for RsaPrivateNumbers (struct)
macro_rules! Depcrate_backend_rsaRsaPrivateNumbers {
() => {
// Module: crate::backend::rsa
// Provides: {"RsaPrivateNumbers"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.primitives.asymmetric.rsa" , name = "RSAPrivateNumbers")] struct RsaPrivateNumbers { # [pyo3 (get)] p : pyo3 :: Py < pyo3 :: types :: PyInt > , # [pyo3 (get)] q : pyo3 :: Py < pyo3 :: types :: PyInt > , # [pyo3 (get)] d : pyo3 :: Py < pyo3 :: types :: PyInt > , # [pyo3 (get)] dmp1 : pyo3 :: Py < pyo3 :: types :: PyInt > , # [pyo3 (get)] dmq1 : pyo3 :: Py < pyo3 :: types :: PyInt > , # [pyo3 (get)] iqmp : pyo3 :: Py < pyo3 :: types :: PyInt > , # [pyo3 (get)] public_numbers : pyo3 :: Py < RsaPublicNumbers > , }
};
}
