// Generated macro for __unless_target_features (macro)
macro_rules! Depcrate_aarch64__unless_target_features {
() => {
// Module: crate::aarch64
// Provides: {"__unless_target_features"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! __unless_target_features { ($ ($ tf : tt) ,+ => $ body : expr) => { { # [cfg (not (all ($ (target_feature =$ tf ,) *)))] $ body # [cfg (all ($ (target_feature =$ tf ,) *))] true } } ; }
};
}
