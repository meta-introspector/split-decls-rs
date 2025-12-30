// Generated macro for CffiBuf (struct)
macro_rules! Depcrate_bufCffiBuf {
() => {
// Module: crate::buf
// Provides: {"CffiBuf"}
// Dependencies: {}
pub (crate) struct CffiBuf < 'p > { pyobj : pyo3 :: Bound < 'p , pyo3 :: PyAny > , # [cfg (not (Py_3_11))] _bufobj : pyo3 :: Bound < 'p , pyo3 :: PyAny > , # [cfg (Py_3_11)] _bufobj : Option < pyo3 :: buffer :: PyBuffer < u8 > > , buf : & 'p [u8] , }
};
}
