// Generated macro for cargo_cfg_var (function)
macro_rules! Depcrate_inputcargo_cfg_var {
() => {
// Module: crate::input
// Provides: {"cargo_cfg_var"}
// Dependencies: {}
# [track_caller] fn cargo_cfg_var (cfg : & str) -> String { if ! is_ascii_ident (cfg) { panic ! ("invalid configuration option {cfg:?}") } let cfg = cfg . to_uppercase () . replace ('-' , "_") ; let key = format ! ("CARGO_CFG_{cfg}") ; key }
};
}
