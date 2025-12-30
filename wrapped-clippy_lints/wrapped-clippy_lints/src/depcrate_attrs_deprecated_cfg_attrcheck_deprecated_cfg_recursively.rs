// Generated macro for check_deprecated_cfg_recursively (function)
macro_rules! Depcrate_attrs_deprecated_cfg_attrcheck_deprecated_cfg_recursively {
() => {
// Module: crate::attrs::deprecated_cfg_attr
// Provides: {"check_deprecated_cfg_recursively"}
// Dependencies: {}
fn check_deprecated_cfg_recursively (cx : & EarlyContext < '_ > , attr : & rustc_ast :: MetaItem) { if let Some (ident) = attr . ident () { if matches ! (ident . name , sym :: any | sym :: all | sym :: not) { let Some (list) = attr . meta_item_list () else { return } ; for item in list . iter () . filter_map (| item | item . meta_item ()) { check_deprecated_cfg_recursively (cx , item) ; } } else { check_cargo_clippy_attr (cx , attr) ; } } }
};
}
