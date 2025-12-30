// Generated macro for ConcatKdfHmac (struct)
macro_rules! Depcrate_backend_kdfConcatKdfHmac {
() => {
// Module: crate::backend::kdf
// Provides: {"ConcatKdfHmac"}
// Dependencies: {}
# [pyo3 :: pyclass (module = "cryptography.hazmat.primitives.kdf.concatkdf" , name = "ConcatKDFHMAC")] struct ConcatKdfHmac { algorithm : pyo3 :: Py < pyo3 :: PyAny > , length : usize , salt : pyo3 :: Py < pyo3 :: types :: PyBytes > , otherinfo : Option < pyo3 :: Py < pyo3 :: types :: PyBytes > > , used : bool , }
};
}
