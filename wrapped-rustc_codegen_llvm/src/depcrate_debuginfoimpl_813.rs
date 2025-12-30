// Generated macro for impl_813 (impl)
macro_rules! Depcrate_debuginfoimpl_813 {
() => {
// Module: crate::debuginfo
// Provides: {"impl_813"}
// Dependencies: {}
impl < 'll > CodegenCx < 'll , '_ > { # [doc = " Looks up debug source information about a `BytePos`."] fn lookup_debug_loc (& self , pos : BytePos) -> DebugLoc { let (file , line , col) = match self . sess () . source_map () . lookup_line (pos) { Ok (SourceFileAndLine { sf : file , line }) => { let line_pos = file . lines () [line] ; let line = (line + 1) as u32 ; let col = (file . relative_position (pos) - line_pos) . to_u32 () + 1 ; (file , line , col) } Err (file) => (file , UNKNOWN_LINE_NUMBER , UNKNOWN_COLUMN_NUMBER) , } ; if self . sess () . target . is_like_msvc { DebugLoc { file , line , col : UNKNOWN_COLUMN_NUMBER } } else { DebugLoc { file , line , col } } } fn create_template_type_parameter (& self , name : & str , actual_type_metadata : & 'll DIType ,) -> & 'll DITemplateTypeParameter { unsafe { llvm :: LLVMRustDIBuilderCreateTemplateTypeParameter (DIB (self) , None , name . as_c_char_ptr () , name . len () , actual_type_metadata ,) } } }
};
}
