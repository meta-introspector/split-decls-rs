// Generated macro for impl_396 (impl)
macro_rules! Depcrate_bufimpl_396 {
() => {
// Module: crate::buf
// Provides: {"impl_396"}
// Dependencies: {}
impl < 'a > CffiMutBuf < 'a > { pub (crate) fn from_bytes (py : pyo3 :: Python < 'a > , buf : & 'a mut [u8]) -> Self { CffiMutBuf { _pyobj : py . None () . into_bound (py) , # [cfg (Py_3_11)] _bufobj : None , # [cfg (not (Py_3_11))] _bufobj : py . None () . into_bound (py) , buf , } } pub (crate) fn as_mut_bytes (& mut self) -> & mut [u8] { self . buf } }
};
}
