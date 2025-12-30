// Generated macro for impl_394 (impl)
macro_rules! Depcrate_bufimpl_394 {
() => {
// Module: crate::buf
// Provides: {"impl_394"}
// Dependencies: {}
impl < 'p > pyo3 :: conversion :: FromPyObject < '_ , 'p > for CffiBuf < 'p > { type Error = pyo3 :: PyErr ; fn extract (pyobj : pyo3 :: Borrowed < '_ , 'p , pyo3 :: PyAny >) -> pyo3 :: PyResult < Self > { let (bufobj , ptrval , len) = _extract_buffer_length (& pyobj , false) ? ; let buf = if len == 0 { & [] } else { unsafe { slice :: from_raw_parts (ptrval as * const u8 , len) } } ; Ok (CffiBuf { pyobj : pyobj . to_owned () , _bufobj : bufobj , buf , }) } }
};
}
