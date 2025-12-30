// Generated macro for impl_347 (impl)
macro_rules! Depcrate_debuginfoimpl_347 {
() => {
// Module: crate::debuginfo
// Provides: {"impl_347"}
// Dependencies: {}
impl < 'gcc , 'tcx > CodegenCx < 'gcc , 'tcx > { # [doc = " Looks up debug source information about a `BytePos`."] pub fn lookup_debug_loc (& self , pos : BytePos) -> DebugLoc { let (file , line , col) = match self . sess () . source_map () . lookup_line (pos) { Ok (SourceFileAndLine { sf : file , line }) => { let line_pos = file . lines () [line] ; let line = (line + 1) as u32 ; let col = (file . relative_position (pos) - line_pos) . to_u32 () + 1 ; (file , line , col) } Err (file) => (file , UNKNOWN_LINE_NUMBER , UNKNOWN_COLUMN_NUMBER) , } ; if self . sess () . target . is_like_msvc { DebugLoc { file , line , col : UNKNOWN_COLUMN_NUMBER } } else { DebugLoc { file , line , col } } } }
};
}
