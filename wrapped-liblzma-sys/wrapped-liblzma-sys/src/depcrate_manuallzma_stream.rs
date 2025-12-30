// Generated macro for lzma_stream (struct)
macro_rules! Depcrate_manuallzma_stream {
() => {
// Module: crate::manual
// Provides: {"lzma_stream"}
// Dependencies: {}
# [repr (C)] pub struct lzma_stream { pub next_in : * const u8 , pub avail_in : size_t , pub total_in : u64 , pub next_out : * mut u8 , pub avail_out : size_t , pub total_out : u64 , pub allocator : * const lzma_allocator , internal : * mut lzma_internal , reserved_ptr1 : * mut c_void , reserved_ptr2 : * mut c_void , reserved_ptr3 : * mut c_void , reserved_ptr4 : * mut c_void , reserved_int1 : u64 , reserved_int2 : u64 , reserved_int3 : size_t , reserved_int4 : size_t , reserved_enum1 : lzma_reserved_enum , reserved_enum2 : lzma_reserved_enum , }
};
}
