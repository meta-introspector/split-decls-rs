// Generated macro for impl_580 (impl)
macro_rules! Depcrate_options_from_metaimpl_580 {
() => {
// Module: crate::options::from_meta
// Provides: {"impl_580"}
// Dependencies: {}
impl ParseAttribute for FromMetaOptions { fn parse_nested (& mut self , mi : & syn :: Meta) -> Result < () > { let path = mi . path () ; if path . is_ident ("from_word") { if self . from_word . is_some () { return Err (Error :: duplicate_field_path (path) . with_span (path)) ; } self . from_word = FromMeta :: from_meta (mi) . map (Some) ? ; } else if path . is_ident ("from_none") { if self . from_none . is_some () { return Err (Error :: duplicate_field_path (path) . with_span (path)) ; } self . from_none = FromMeta :: from_meta (mi) . map (Some) ? ; } else if path . is_ident ("from_expr") { if self . from_expr . is_some () { return Err (Error :: duplicate_field_path (path) . with_span (path)) ; } self . from_expr = FromMeta :: from_meta (mi) . map (Some) ? ; } else if path . is_ident ("derive_syn_parse") { if self . derive_syn_parse . is_some () { return Err (Error :: duplicate_field_path (path) . with_span (path)) ; } self . derive_syn_parse = FromMeta :: from_meta (mi) . map (Some) ? ; } else { self . base . parse_nested (mi) ? ; } Ok (()) } }
};
}
