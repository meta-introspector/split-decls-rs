// Generated macro for get_ex_data_from_ptr (function)
macro_rules! Depcrate_tlsget_ex_data_from_ptr {
() => {
// Module: crate::tls
// Provides: {"get_ex_data_from_ptr"}
// Dependencies: {}
fn get_ex_data_from_ptr < 'a , T > (ptr : * const SSL , idx : c_int) -> Option < & 'a mut T > { unsafe { let data = SSL_get_ex_data (ptr , idx) as * mut T ; data . as_mut () } }
};
}
