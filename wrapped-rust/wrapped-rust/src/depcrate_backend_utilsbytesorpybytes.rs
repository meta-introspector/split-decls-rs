// Generated macro for BytesOrPyBytes (enum)
macro_rules! Depcrate_backend_utilsBytesOrPyBytes {
() => {
// Module: crate::backend::utils
// Provides: {"BytesOrPyBytes"}
// Dependencies: {}
pub (crate) enum BytesOrPyBytes < 'a > { Bytes (& 'a [u8]) , PyBytes (pyo3 :: Bound < 'a , pyo3 :: types :: PyBytes >) , }
};
}
