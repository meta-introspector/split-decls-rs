// Generated macro for apply_name_type (function)
macro_rules! Depcrate_coff_import_fileapply_name_type {
() => {
// Module: crate::coff_import_file
// Provides: {"apply_name_type"}
// Dependencies: {}
fn apply_name_type (import_type : ImportNameType , name : & str) -> & str { fn ltrim1 < 'a > (name : & 'a str , chars : & str) -> & 'a str { if let Some ((first_char , rest)) = name . split_at_checked (1) && chars . contains (first_char) { return rest ; } name } match import_type { ImportNameType :: NameNoprefix => ltrim1 (name , "?@_") , ImportNameType :: NameUndecorate => { let name = ltrim1 (name , "?@_") ; & name [.. name . find ('@') . unwrap_or (name . len ())] } _ => name , } }
};
}
