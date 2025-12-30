// Generated macro for _extract_buffer_length (function)
macro_rules! Depcrate_buf_extract_buffer_length {
() => {
// Module: crate::buf
// Provides: {"_extract_buffer_length"}
// Dependencies: {}
# [cfg (not (Py_3_11))] fn _extract_buffer_length < 'p > (pyobj : & pyo3 :: Borrowed < '_ , 'p , pyo3 :: PyAny > , mutable : bool ,) -> pyo3 :: PyResult < (pyo3 :: Bound < 'p , pyo3 :: PyAny > , usize , usize) > { let py = pyobj . py () ; let bufobj = if mutable { let kwargs = [(pyo3 :: intern ! (py , "require_writable") , true)] ; let kwargs = pyo3 :: types :: IntoPyDict :: into_py_dict (kwargs , py) ? ; types :: FFI_FROM_BUFFER . get (py) ? . call ((pyobj ,) , Some (& kwargs)) } else { types :: FFI_FROM_BUFFER . get (py) ? . call1 ((pyobj ,)) } . map_err (| _ | { let errmsg = generate_non_convertible_buffer_error_msg (pyobj) ; pyo3 :: exceptions :: PyTypeError :: new_err (errmsg) }) ? ; let ptrval = types :: FFI_CAST . get (py) ? . call1 ((pyo3 :: intern ! (py , "uintptr_t") , bufobj . clone ())) ? . call_method0 (pyo3 :: intern ! (py , "__int__")) ? . extract :: < usize > () ? ; let len = bufobj . len () ? ; Ok ((bufobj , ptrval , len)) }
};
}
