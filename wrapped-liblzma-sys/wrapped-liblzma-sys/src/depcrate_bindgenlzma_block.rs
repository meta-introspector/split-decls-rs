// Generated macro for lzma_block (struct)
macro_rules! Depcrate_bindgenlzma_block {
() => {
// Module: crate::bindgen
// Provides: {"lzma_block"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct lzma_block { pub version : u32 , pub header_size : u32 , pub check : lzma_check , pub compressed_size : lzma_vli , pub uncompressed_size : lzma_vli , pub filters : * mut lzma_filter , pub raw_check : [u8 ; 64usize] , pub reserved_ptr1 : * mut :: std :: os :: raw :: c_void , pub reserved_ptr2 : * mut :: std :: os :: raw :: c_void , pub reserved_ptr3 : * mut :: std :: os :: raw :: c_void , pub reserved_int1 : u32 , pub reserved_int2 : u32 , pub reserved_int3 : lzma_vli , pub reserved_int4 : lzma_vli , pub reserved_int5 : lzma_vli , pub reserved_int6 : lzma_vli , pub reserved_int7 : lzma_vli , pub reserved_int8 : lzma_vli , pub reserved_enum1 : lzma_reserved_enum , pub reserved_enum2 : lzma_reserved_enum , pub reserved_enum3 : lzma_reserved_enum , pub reserved_enum4 : lzma_reserved_enum , pub ignore_check : lzma_bool , pub reserved_bool2 : lzma_bool , pub reserved_bool3 : lzma_bool , pub reserved_bool4 : lzma_bool , pub reserved_bool5 : lzma_bool , pub reserved_bool6 : lzma_bool , pub reserved_bool7 : lzma_bool , pub reserved_bool8 : lzma_bool , }
};
}
