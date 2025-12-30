// Generated macro for impl_629 (impl)
macro_rules! Depcrate_options_input_variantimpl_629 {
() => {
// Module: crate::options::input_variant
// Provides: {"impl_629"}
// Dependencies: {}
impl ParseAttribute for InputVariant { fn parse_nested (& mut self , mi : & syn :: Meta) -> Result < () > { let path = mi . path () ; if path . is_ident ("rename") { if self . attr_name . is_some () { return Err (Error :: duplicate_field_path (path) . with_span (mi)) ; } self . attr_name = FromMeta :: from_meta (mi) ? ; } else if path . is_ident ("skip") { if self . skip . is_some () { return Err (Error :: duplicate_field_path (path) . with_span (mi)) ; } self . skip = FromMeta :: from_meta (mi) ? ; } else if path . is_ident ("word") { if self . word . is_some () { return Err (Error :: duplicate_field_path (path) . with_span (mi)) ; } if ! self . data . is_unit () { let note = "`#[darling(word)]` can only be applied to a unit variant" ; # [cfg (feature = "diagnostics")] let error = Error :: unknown_field_path (path) . note (note) ; # [cfg (not (feature = "diagnostics"))] let error = Error :: custom (format ! ("Unexpected field: `word`. {}" , note)) ; return Err (error . with_span (mi)) ; } self . word = FromMeta :: from_meta (mi) ? ; } else { return Err (Error :: unknown_field_path (path) . with_span (mi)) ; } Ok (()) } }
};
}
