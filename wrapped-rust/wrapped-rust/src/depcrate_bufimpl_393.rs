// Generated macro for impl_393 (impl)
macro_rules! Depcrate_bufimpl_393 {
() => {
// Module: crate::buf
// Provides: {"impl_393"}
// Dependencies: {}
impl < 'a > CffiBuf < 'a > { pub (crate) fn from_bytes (py : pyo3 :: Python < 'a > , buf : & 'a [u8]) -> Self { CffiBuf { pyobj : py . None () . into_bound (py) , # [cfg (Py_3_11)] _bufobj : None , # [cfg (not (Py_3_11))] _bufobj : py . None () . into_bound (py) , buf , } } pub (crate) fn as_bytes (& self) -> & [u8] { self . buf } pub (crate) fn into_pyobj (self) -> pyo3 :: Bound < 'a , pyo3 :: PyAny > { self . pyobj } }
};
}
