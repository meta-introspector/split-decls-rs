// Generated macro for generate_non_convertible_buffer_error_msg (function)
macro_rules! Depcrate_bufgenerate_non_convertible_buffer_error_msg {
() => {
// Module: crate::buf
// Provides: {"generate_non_convertible_buffer_error_msg"}
// Dependencies: {}
fn generate_non_convertible_buffer_error_msg (pyobj : & pyo3 :: Borrowed < '_ , '_ , pyo3 :: PyAny > ,) -> String { if pyobj . is_instance_of :: < pyo3 :: types :: PyString > () { format ! ("Cannot convert \"{}\" instance to a buffer.\nDid you mean to pass a bytestring instead?" , pyobj . get_type ()) } else { format ! ("Cannot convert \"{}\" instance to a buffer." , pyobj . get_type ()) } }
};
}
