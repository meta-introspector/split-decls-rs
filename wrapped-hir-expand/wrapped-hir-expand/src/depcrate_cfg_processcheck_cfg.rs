// Generated macro for check_cfg (function)
macro_rules! Depcrate_cfg_processcheck_cfg {
() => {
// Module: crate::cfg_process
// Provides: {"check_cfg"}
// Dependencies: {}
fn check_cfg (db : & dyn ExpandDatabase , attr : & Attr , krate : Crate) -> Option < bool > { if ! attr . simple_name () . as_deref () . map (| v | v == "cfg") ? { return None ; } let cfg = parse_from_attr_token_tree (& attr . meta () ? . token_tree () ?) ? ; let enabled = krate . cfg_options (db) . check (& cfg) != Some (false) ; Some (enabled) }
};
}
