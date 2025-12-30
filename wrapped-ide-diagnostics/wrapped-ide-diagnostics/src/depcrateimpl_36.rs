// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl DiagnosticsContext < '_ > { fn resolve_precise_location (& self , node : & InFile < SyntaxNodePtr > , precise_location : Option < TextRange > ,) -> FileRange { let sema = & self . sema ; (| | { let precise_location = precise_location ? ; let root = sema . parse_or_expand (node . file_id) ; match root . covering_element (precise_location) { syntax :: NodeOrToken :: Node (it) => Some (sema . original_range (& it)) , syntax :: NodeOrToken :: Token (it) => { node . with_value (it) . original_file_range_opt (sema . db) } } }) () . map (| frange | ide_db :: FileRange { file_id : frange . file_id . file_id (self . sema . db) , range : frange . range , }) . unwrap_or_else (| | sema . diagnostics_display_range (* node)) } }
};
}
