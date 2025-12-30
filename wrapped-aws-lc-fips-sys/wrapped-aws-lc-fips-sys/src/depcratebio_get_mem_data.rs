// Generated macro for BIO_get_mem_data (function)
macro_rules! DepcrateBIO_get_mem_data {
() => {
// Module: crate
// Provides: {"BIO_get_mem_data"}
// Dependencies: {}
# [allow (non_snake_case , clippy :: not_unsafe_ptr_arg_deref)] pub fn BIO_get_mem_data (b : * mut BIO , pp : * mut * mut c_char) -> c_long { unsafe { BIO_ctrl (b , BIO_CTRL_INFO , 0 , pp . cast :: < c_void > ()) } }
};
}
