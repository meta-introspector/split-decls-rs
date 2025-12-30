// Generated macro for check (function)
macro_rules! Depcrate_attrs_non_minimal_cfgcheck {
() => {
// Module: crate::attrs::non_minimal_cfg
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & EarlyContext < '_ > , attr : & Attribute) { if attr . has_name (sym :: cfg) && let Some (items) = attr . meta_item_list () { check_nested_cfg (cx , & items) ; } }
};
}
