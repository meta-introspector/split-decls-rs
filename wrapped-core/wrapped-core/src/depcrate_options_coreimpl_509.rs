// Generated macro for impl_509 (impl)
macro_rules! Depcrate_options_coreimpl_509 {
() => {
// Module: crate::options::core
// Provides: {"impl_509"}
// Dependencies: {}
impl ParseAttribute for Core { fn parse_nested (& mut self , mi : & syn :: Meta) -> Result < () > { let path = mi . path () ; if path . is_ident ("default") { if self . default . is_some () { return Err (Error :: duplicate_field ("default") . with_span (mi)) ; } self . default = FromMeta :: from_meta (mi) ? ; } else if path . is_ident ("rename_all") { self . rename_rule = FromMeta :: from_meta (mi) ? ; } else if path . is_ident ("map") || path . is_ident ("and_then") { let transformer = path . get_ident () . unwrap () . clone () ; if let Some (post_transform) = & self . post_transform { if transformer == post_transform . transformer { return Err (Error :: duplicate_field (& transformer . to_string ()) . with_span (mi)) ; } else { return Err (Error :: custom (format ! ("Options `{}` and `{}` are mutually exclusive" , transformer , post_transform . transformer)) . with_span (mi)) ; } } self . post_transform = Some (PostfixTransform :: new (transformer , FromMeta :: from_meta (mi) ?)) ; } else if path . is_ident ("bound") { self . bound = FromMeta :: from_meta (mi) ? ; } else if path . is_ident ("allow_unknown_fields") { if self . allow_unknown_fields . is_some () { return Err (Error :: duplicate_field ("allow_unknown_fields") . with_span (mi)) ; } self . allow_unknown_fields = FromMeta :: from_meta (mi) ? ; } else { return Err (Error :: unknown_field_path (path) . with_span (mi)) ; } Ok (()) } }
};
}
