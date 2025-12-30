// Generated macro for macro_64 (macro)
macro_rules! Depcrate_tdefmacro_64 {
() => {
// Module: crate::tdef
// Provides: {"macro_64"}
// Dependencies: {}
unmangle ! (pub unsafe extern "C" fn tdefl_compress_mem_to_heap (src_buf : * const c_void , src_buf_len : usize , out_len : * mut usize , flags : c_int ,) -> * mut c_void { match out_len . as_mut () { None => ptr :: null_mut () , Some (len) => { * len = 0 ; let mut buffer_user = BufferUser { size : 0 , capacity : 0 , buf : ptr :: null_mut () , expandable : true , } ; if tdefl_compress_mem_to_output (src_buf , src_buf_len , Some (output_buffer_putter) , & mut buffer_user as * mut BufferUser as * mut c_void , flags ,) == 0 { ptr :: null_mut () } else { * len = buffer_user . size ; buffer_user . buf as * mut c_void } } } } pub unsafe extern "C" fn tdefl_compress_mem_to_mem (out_buf : * mut c_void , out_buf_len : usize , src_buf : * const c_void , src_buf_len : usize , flags : c_int ,) -> usize { if out_buf . is_null () { return 0 ; } let mut buffer_user = BufferUser { size : 0 , capacity : out_buf_len , buf : out_buf as * mut u8 , expandable : false , } ; if tdefl_compress_mem_to_output (src_buf , src_buf_len , Some (output_buffer_putter) , & mut buffer_user as * mut BufferUser as * mut c_void , flags ,) != 0 { buffer_user . size } else { 0 } } pub extern "C" fn tdefl_create_comp_flags_from_zip_params (level : c_int , window_bits : c_int , strategy : c_int ,) -> c_uint { create_comp_flags_from_zip_params (level , window_bits , strategy) }) ;
};
}
