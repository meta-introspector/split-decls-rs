// Generated macro for lzma_options_lzma (struct)
macro_rules! Depcrate_bindgenlzma_options_lzma {
() => {
// Module: crate::bindgen
// Provides: {"lzma_options_lzma"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct lzma_options_lzma { pub dict_size : u32 , pub preset_dict : * const u8 , pub preset_dict_size : u32 , pub lc : u32 , pub lp : u32 , pub pb : u32 , pub mode : lzma_mode , pub nice_len : u32 , pub mf : lzma_match_finder , pub depth : u32 , pub ext_flags : u32 , pub ext_size_low : u32 , pub ext_size_high : u32 , pub reserved_int4 : u32 , pub reserved_int5 : u32 , pub reserved_int6 : u32 , pub reserved_int7 : u32 , pub reserved_int8 : u32 , pub reserved_enum1 : lzma_reserved_enum , pub reserved_enum2 : lzma_reserved_enum , pub reserved_enum3 : lzma_reserved_enum , pub reserved_enum4 : lzma_reserved_enum , pub reserved_ptr1 : * mut :: std :: os :: raw :: c_void , pub reserved_ptr2 : * mut :: std :: os :: raw :: c_void , }
};
}
