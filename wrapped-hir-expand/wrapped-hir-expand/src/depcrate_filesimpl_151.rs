// Generated macro for impl_151 (impl)
macro_rules! Depcrate_filesimpl_151 {
() => {
// Module: crate::files
// Provides: {"impl_151"}
// Dependencies: {}
impl InMacroFile < TextSize > { pub fn original_file_range (self , db : & dyn db :: ExpandDatabase) -> (FileRange , SyntaxContext) { span_for_offset (db , & db . expansion_span_map (self . file_id) , self . value) } }
};
}
