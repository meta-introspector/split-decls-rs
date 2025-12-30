// Generated macro for Hkdf (struct)
macro_rules! Depcrate_backend_kdfHkdf {
() => {
// Module: crate::backend::kdf
// Provides: {"Hkdf"}
// Dependencies: {}
# [pyo3 :: pyclass (module = "cryptography.hazmat.primitives.kdf.hkdf" , name = "HKDF")] struct Hkdf { algorithm : pyo3 :: Py < pyo3 :: PyAny > , salt : Option < pyo3 :: Py < pyo3 :: types :: PyBytes > > , info : Option < pyo3 :: Py < pyo3 :: types :: PyBytes > > , length : usize , used : bool , }
};
}
