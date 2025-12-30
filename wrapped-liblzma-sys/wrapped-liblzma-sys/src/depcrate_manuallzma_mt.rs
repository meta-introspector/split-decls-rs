// Generated macro for lzma_mt (struct)
macro_rules! Depcrate_manuallzma_mt {
() => {
// Module: crate::manual
// Provides: {"lzma_mt"}
// Dependencies: {}
# [cfg (feature = "parallel")] # [repr (C)] pub struct lzma_mt { pub flags : u32 , pub threads : u32 , pub block_size : u64 , pub timeout : u32 , pub preset : u32 , pub filters : * const lzma_filter , pub check : lzma_check , reserved_enum1 : lzma_reserved_enum , reserved_enum2 : lzma_reserved_enum , reserved_enum3 : lzma_reserved_enum , reserved_int1 : u32 , reserved_int2 : u32 , reserved_int3 : u32 , reserved_int4 : u32 , pub memlimit_threading : u64 , pub memlimit_stop : u64 , reserved_int7 : u64 , reserved_int8 : u64 , reserved_ptr1 : * mut c_void , reserved_ptr2 : * mut c_void , reserved_ptr3 : * mut c_void , reserved_ptr4 : * mut c_void , }
};
}
