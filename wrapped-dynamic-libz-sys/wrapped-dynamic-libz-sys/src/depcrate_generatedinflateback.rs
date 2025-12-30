// Generated macro for inflateBack (function)
macro_rules! Depcrate_generatedinflateBack {
() => {
// Module: crate::generated
// Provides: {"inflateBack"}
// Dependencies: {}
pub unsafe fn inflateBack (strm : z_streamp , _in : in_func , in_desc : * mut c_void , out : out_func , out_desc : * mut c_void ,) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , _in : in_func , in_desc : * mut c_void , out : out_func , out_desc : * mut c_void ,) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "inflateBack") } , "inflateBack") . as_bytes ()) . unwrap () ; f (strm , _in , in_desc , out , out_desc) }
};
}
