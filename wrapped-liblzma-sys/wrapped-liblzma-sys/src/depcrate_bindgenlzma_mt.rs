// Generated macro for lzma_mt (struct)
macro_rules! Depcrate_bindgenlzma_mt {
() => {
// Module: crate::bindgen
// Provides: {"lzma_mt"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct lzma_mt { pub flags : u32 , pub threads : u32 , pub block_size : u64 , pub timeout : u32 , pub preset : u32 , pub filters : * const lzma_filter , pub check : lzma_check , pub reserved_enum1 : lzma_reserved_enum , pub reserved_enum2 : lzma_reserved_enum , pub reserved_enum3 : lzma_reserved_enum , pub reserved_int1 : u32 , pub reserved_int2 : u32 , pub reserved_int3 : u32 , pub reserved_int4 : u32 , pub memlimit_threading : u64 , pub memlimit_stop : u64 , pub reserved_int7 : u64 , pub reserved_int8 : u64 , pub reserved_ptr1 : * mut :: std :: os :: raw :: c_void , pub reserved_ptr2 : * mut :: std :: os :: raw :: c_void , pub reserved_ptr3 : * mut :: std :: os :: raw :: c_void , pub reserved_ptr4 : * mut :: std :: os :: raw :: c_void , }
};
}
