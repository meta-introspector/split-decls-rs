// Generated macro for find_gated_cfg (function)
macro_rules! Depcrate_builtin_attrsfind_gated_cfg {
() => {
// Module: crate::builtin_attrs
// Provides: {"find_gated_cfg"}
// Dependencies: {}
# [doc = " Find a gated cfg determined by the `pred`icate which is given the cfg's name."] pub fn find_gated_cfg (pred : impl Fn (Symbol) -> bool) -> Option < & 'static GatedCfg > { GATED_CFGS . iter () . find (| (cfg_sym , ..) | pred (* cfg_sym)) }
};
}
