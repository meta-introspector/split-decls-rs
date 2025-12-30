// Generated macro for free_data_box (function)
macro_rules! Depcrate_sslfree_data_box {
() => {
// Module: crate::ssl
// Provides: {"free_data_box"}
// Dependencies: {}
unsafe extern "C" fn free_data_box < T > (_parent : * mut c_void , ptr : * mut c_void , _ad : * mut ffi :: CRYPTO_EX_DATA , _idx : c_int , _argl : c_long , _argp : * mut c_void ,) { if ! ptr . is_null () { let _ = Box :: < T > :: from_raw (ptr as * mut T) ; } }
};
}
