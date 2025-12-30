// Generated macro for check_cfg_attr (function)
macro_rules! Depcrate_cfg_processcheck_cfg_attr {
() => {
// Module: crate::cfg_process
// Provides: {"check_cfg_attr"}
// Dependencies: {}
fn check_cfg_attr (db : & dyn ExpandDatabase , attr : & Attr , krate : Crate) -> Option < bool > { if ! attr . simple_name () . as_deref () . map (| v | v == "cfg_attr") ? { return None ; } check_cfg_attr_value (db , & attr . token_tree () ? , krate) }
};
}
