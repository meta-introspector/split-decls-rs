// Generated macro for impl_149 (impl)
macro_rules! Depcrate_filesimpl_149 {
() => {
// Module: crate::files
// Provides: {"impl_149"}
// Dependencies: {}
impl InMacroFile < SyntaxToken > { pub fn upmap_once (self , db : & dyn db :: ExpandDatabase ,) -> InFile < smallvec :: SmallVec < TextRange , 1 > > { self . file_id . expansion_info (db) . map_range_up_once (db , self . value . text_range ()) } }
};
}
