// Generated macro for eq_mac_call (function)
macro_rules! Depcrate_ast_utilseq_mac_call {
() => {
// Module: crate::ast_utils
// Provides: {"eq_mac_call"}
// Dependencies: {}
pub fn eq_mac_call (l : & MacCall , r : & MacCall) -> bool { eq_path (& l . path , & r . path) && eq_delim_args (& l . args , & r . args) }
};
}
