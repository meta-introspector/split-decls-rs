// Generated macro for try_gate_cfg (function)
macro_rules! Depcrate_attributes_cfg_oldtry_gate_cfg {
() => {
// Module: crate::attributes::cfg_old
// Provides: {"try_gate_cfg"}
// Dependencies: {}
pub fn try_gate_cfg (name : Symbol , span : Span , sess : & Session , features : Option < & Features >) { let gate = find_gated_cfg (| sym | sym == name) ; if let (Some (feats) , Some (gated_cfg)) = (features , gate) { gate_cfg (gated_cfg , span , sess , feats) ; } }
};
}
