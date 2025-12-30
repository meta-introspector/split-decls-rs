// Generated macro for mz_stream (struct)
macro_rules! Depcrate_c_exportmz_stream {
() => {
// Module: crate::c_export
// Provides: {"mz_stream"}
// Dependencies: {}
# [doc = " Inner stream state containing pointers to the used buffers and internal state."] # [repr (C)] # [allow (bad_style)] # [derive (Debug)] pub struct mz_stream { # [doc = " Pointer to the current start of the input buffer."] pub next_in : * const u8 , # [doc = " Length of the input buffer."] pub avail_in : c_uint , # [doc = " The total number of input bytes consumed so far."] pub total_in : c_ulong , # [doc = " Pointer to the current start of the output buffer."] pub next_out : * mut u8 , # [doc = " Space in the output buffer."] pub avail_out : c_uint , # [doc = " The total number of bytes output so far."] pub total_out : c_ulong , pub msg : * const c_char , # [doc = " Compressor or decompressor, if it exists."] # [doc = " This is boxed to work with the current C API."] pub state : Option < Box < InternalState > > , # [doc = " Allocation function to use for allocating the internal compressor/decompressor."] # [doc = " Uses `mz_default_alloc_func` if set to `None`."] pub zalloc : mz_alloc_callback , # [doc = " Free function to use for allocating the internal compressor/decompressor."] # [doc = " Uses `mz_default_free_func` if `None`."] pub zfree : mz_free_callback , # [doc = " Extra data to provide the allocation/deallocation functions."] # [doc = " (Not used for the default ones)"] pub opaque : * mut c_void , # [doc = " Whether the stream contains a compressor or decompressor."] pub data_type : StateTypeEnum , # [doc = " Adler32 checksum of the data that has been compressed or uncompressed."] pub adler : c_ulong , # [doc = " Reserved"] pub reserved : c_ulong , }
};
}
