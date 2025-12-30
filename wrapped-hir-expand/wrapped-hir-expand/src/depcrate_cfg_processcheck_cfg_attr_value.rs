// Generated macro for check_cfg_attr_value (function)
macro_rules! Depcrate_cfg_processcheck_cfg_attr_value {
() => {
// Module: crate::cfg_process
// Provides: {"check_cfg_attr_value"}
// Dependencies: {}
pub fn check_cfg_attr_value (db : & dyn ExpandDatabase , attr : & TokenTree , krate : Crate ,) -> Option < bool > { let cfg_expr = parse_from_attr_token_tree (attr) ? ; let enabled = krate . cfg_options (db) . check (& cfg_expr) != Some (false) ; Some (enabled) }
};
}
