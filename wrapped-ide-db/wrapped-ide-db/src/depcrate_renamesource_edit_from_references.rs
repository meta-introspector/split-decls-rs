// Generated macro for source_edit_from_references (function)
macro_rules! Depcrate_renamesource_edit_from_references {
() => {
// Module: crate::rename
// Provides: {"source_edit_from_references"}
// Dependencies: {}
pub fn source_edit_from_references (db : & RootDatabase , references : & [FileReference] , def : Definition , new_name : & Name , edition : Edition ,) -> TextEdit { let name_display = new_name . display (db , edition) ; let mut edit = TextEdit :: builder () ; let mut edited_ranges = Vec :: new () ; for & FileReference { range , ref name , .. } in references { let name_range = name . text_range () ; let has_emitted_edit = match name { FileReferenceNode :: NameRef (name_ref) if name_range == range => { source_edit_from_name_ref (& mut edit , name_ref , & name_display , def) } FileReferenceNode :: Name (name) if name_range == range => { source_edit_from_name (& mut edit , name , & name_display) } _ => false , } ; if ! has_emitted_edit && ! edited_ranges . contains (& range . start ()) { edit . replace (range , name_display . to_string ()) ; edited_ranges . push (range . start ()) ; } } edit . finish () }
};
}
