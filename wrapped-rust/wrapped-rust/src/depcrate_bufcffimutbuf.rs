// Generated macro for CffiMutBuf (struct)
macro_rules! Depcrate_bufCffiMutBuf {
() => {
// Module: crate::buf
// Provides: {"CffiMutBuf"}
// Dependencies: {}
pub (crate) struct CffiMutBuf < 'p > { _pyobj : pyo3 :: Bound < 'p , pyo3 :: PyAny > , # [cfg (not (Py_3_11))] _bufobj : pyo3 :: Bound < 'p , pyo3 :: PyAny > , # [cfg (Py_3_11)] _bufobj : Option < pyo3 :: buffer :: PyBuffer < u8 > > , buf : & 'p mut [u8] , }
};
}
