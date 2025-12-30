// Generated macro for test (module)
macro_rules! Depcrate_tdeftest {
() => {
// Module: crate::tdef
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use miniz_oxide :: inflate :: decompress_to_vec ; # [test] fn mem_to_heap () { let data = b"blargharghawrf31086t13qa9pt7gnseatgawe78vtb6p71v" ; let mut out_len = 0 ; let data_len = data . len () ; let out_data = unsafe { let res = tdefl_compress_mem_to_heap (data . as_ptr () as * const c_void , data_len , & mut out_len , 0 ,) ; assert ! (! res . is_null ()) ; res } ; { let out_slice = unsafe { slice :: from_raw_parts (out_data as * const u8 , out_len) } ; let dec = decompress_to_vec (out_slice) . unwrap () ; assert ! (dec . as_slice () == & data [..]) ; } } }
};
}
