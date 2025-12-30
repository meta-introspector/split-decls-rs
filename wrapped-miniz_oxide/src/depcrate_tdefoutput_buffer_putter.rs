// Generated macro for output_buffer_putter (function)
macro_rules! Depcrate_tdefoutput_buffer_putter {
() => {
// Module: crate::tdef
// Provides: {"output_buffer_putter"}
// Dependencies: {}
pub unsafe extern "C" fn output_buffer_putter (buf : * const c_void , len : c_int , user : * mut c_void ,) -> i32 { let user = (user as * mut BufferUser) . as_mut () ; match user { None => false . into () , Some (user) => { let new_size = user . size + len as usize ; if new_size > user . capacity { if ! user . expandable { return false . into () ; } let mut new_capacity = cmp :: max (user . capacity , 128) ; while new_size > new_capacity { new_capacity <<= 1 ; } let new_buf = crate :: miniz_def_realloc_func (ptr :: null_mut () , user . buf as * mut c_void , 1 , new_capacity ,) ; if new_buf . is_null () { return false . into () ; } user . buf = new_buf as * mut u8 ; user . capacity = new_capacity ; } ptr :: copy_nonoverlapping (buf as * const u8 , user . buf . add (user . size) , len as usize) ; user . size = new_size ; true . into () } } }
};
}
