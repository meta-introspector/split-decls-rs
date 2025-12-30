// Generated macro for EllipticCurvePrivateNumbers (struct)
macro_rules! Depcrate_backend_ecEllipticCurvePrivateNumbers {
() => {
// Module: crate::backend::ec
// Provides: {"EllipticCurvePrivateNumbers"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.primitives.asymmetric.ec")] struct EllipticCurvePrivateNumbers { # [pyo3 (get)] private_value : pyo3 :: Py < pyo3 :: types :: PyInt > , # [pyo3 (get)] public_numbers : pyo3 :: Py < EllipticCurvePublicNumbers > , }
};
}
