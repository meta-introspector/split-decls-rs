// Generated macro for lzma_stream (struct)
macro_rules! Depcrate_bindgenlzma_stream {
() => {
// Module: crate::bindgen
// Provides: {"lzma_stream"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct lzma_stream { pub next_in : * const u8 , pub avail_in : usize , pub total_in : u64 , pub next_out : * mut u8 , pub avail_out : usize , pub total_out : u64 , pub allocator : * const lzma_allocator , pub internal : * mut lzma_internal , pub reserved_ptr1 : * mut :: std :: os :: raw :: c_void , pub reserved_ptr2 : * mut :: std :: os :: raw :: c_void , pub reserved_ptr3 : * mut :: std :: os :: raw :: c_void , pub reserved_ptr4 : * mut :: std :: os :: raw :: c_void , pub seek_pos : u64 , pub reserved_int2 : u64 , pub reserved_int3 : usize , pub reserved_int4 : usize , pub reserved_enum1 : lzma_reserved_enum , pub reserved_enum2 : lzma_reserved_enum , }
};
}
