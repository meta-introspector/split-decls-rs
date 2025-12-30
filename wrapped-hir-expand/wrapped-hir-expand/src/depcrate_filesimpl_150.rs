// Generated macro for impl_150 (impl)
macro_rules! Depcrate_filesimpl_150 {
() => {
// Module: crate::files
// Provides: {"impl_150"}
// Dependencies: {}
impl InFile < SyntaxToken > { # [doc = " Falls back to the macro call range if the node cannot be mapped up fully."] pub fn original_file_range (self , db : & dyn db :: ExpandDatabase) -> FileRange { match self . file_id { HirFileId :: FileId (file_id) => FileRange { file_id , range : self . value . text_range () } , HirFileId :: MacroFile (mac_file) => { let (range , ctxt) = span_for_offset (db , & db . expansion_span_map (mac_file) , self . value . text_range () . start () ,) ; if ctxt . is_root () { return range ; } let loc = db . lookup_intern_macro_call (mac_file) ; loc . kind . original_call_range (db) } } } # [doc = " Attempts to map the syntax node back up its macro calls."] pub fn original_file_range_opt (self , db : & dyn db :: ExpandDatabase) -> Option < FileRange > { match self . file_id { HirFileId :: FileId (file_id) => { Some (FileRange { file_id , range : self . value . text_range () }) } HirFileId :: MacroFile (mac_file) => { let (range , ctxt) = span_for_offset (db , & db . expansion_span_map (mac_file) , self . value . text_range () . start () ,) ; if ctxt . is_root () { Some (range) } else { None } } } } }
};
}
