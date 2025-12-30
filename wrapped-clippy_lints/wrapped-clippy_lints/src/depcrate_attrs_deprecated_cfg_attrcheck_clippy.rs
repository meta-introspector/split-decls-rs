// Generated macro for check_clippy (function)
macro_rules! Depcrate_attrs_deprecated_cfg_attrcheck_clippy {
() => {
// Module: crate::attrs::deprecated_cfg_attr
// Provides: {"check_clippy"}
// Dependencies: {}
pub (super) fn check_clippy (cx : & EarlyContext < '_ > , attr : & Attribute) { if attr . has_name (sym :: cfg) && let Some (list) = attr . meta_item_list () { for item in list . iter () . filter_map (| item | item . meta_item ()) { check_deprecated_cfg_recursively (cx , item) ; } } }
};
}
