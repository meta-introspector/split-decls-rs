// Generated macro for inflateBack (function)
macro_rules! DepcrateinflateBack {
() => {
// Module: crate
// Provides: {"inflateBack"}
// Dependencies: {}
# [doc = " Decompresses as much data as possible, and stops when the input buffer becomes empty or the output buffer becomes full."] # [doc = ""] # [doc = " ## Safety"] # [doc = ""] # [doc = " The caller must guarantee that"] # [doc = ""] # [doc = " * Either"] # [doc = "     - `strm` is `NULL`"] # [doc = "     - `strm` satisfies the requirements of `&mut *strm` and was initialized with [`inflateBackInit_`]"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (inflateBack))] pub unsafe extern "C-unwind" fn inflateBack (_strm : z_streamp , _in : in_func , _in_desc : * mut c_void , _out : out_func , _out_desc : * mut c_void ,) -> c_int { todo ! ("inflateBack is not implemented yet") }
};
}
