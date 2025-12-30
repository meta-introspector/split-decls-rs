// Generated macro for z_stream (struct)
macro_rules! Depcrate_generatedz_stream {
() => {
// Module: crate::generated
// Provides: {"z_stream"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Clone)] pub struct z_stream { pub next_in : * mut Bytef , pub avail_in : uInt , pub total_in : z_size , pub next_out : * mut Bytef , pub avail_out : uInt , pub total_out : z_size , pub msg : * mut c_char , pub state : * mut internal_state , pub zalloc : alloc_func , pub zfree : free_func , pub opaque : voidpf , pub data_type : c_int , pub adler : z_checksum , pub reserved : uLong , }
};
}
