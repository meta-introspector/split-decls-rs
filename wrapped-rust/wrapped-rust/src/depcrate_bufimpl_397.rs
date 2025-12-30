// Generated macro for impl_397 (impl)
macro_rules! Depcrate_bufimpl_397 {
() => {
// Module: crate::buf
// Provides: {"impl_397"}
// Dependencies: {}
impl < 'p > pyo3 :: conversion :: FromPyObject < '_ , 'p > for CffiMutBuf < 'p > { type Error = pyo3 :: PyErr ; fn extract (pyobj : pyo3 :: Borrowed < '_ , 'p , pyo3 :: PyAny >) -> pyo3 :: PyResult < Self > { let (bufobj , ptrval , len) = _extract_buffer_length (& pyobj , true) ? ; let buf = if len == 0 { & mut [] } else { unsafe { slice :: from_raw_parts_mut (ptrval as * mut u8 , len) } } ; Ok (CffiMutBuf { _pyobj : pyobj . to_owned () , _bufobj : bufobj , buf , }) } }
};
}
