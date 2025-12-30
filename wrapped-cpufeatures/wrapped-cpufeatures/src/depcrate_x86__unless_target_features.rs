// Generated macro for __unless_target_features (macro)
macro_rules! Depcrate_x86__unless_target_features {
() => {
// Module: crate::x86
// Provides: {"__unless_target_features"}
// Dependencies: {}
# [doc = " Evaluate the given `$body` expression any of the supplied target features"] # [doc = " are not enabled. Otherwise returns true."] # [doc = ""] # [doc = " The `$body` expression is not evaluated on SGX targets, and returns false"] # [doc = " on these targets unless *all* supplied target features are enabled."] # [macro_export] # [doc (hidden)] macro_rules ! __unless_target_features { ($ ($ tf : tt) ,+ => $ body : expr) => { { # [cfg (not (all ($ (target_feature =$ tf ,) *)))] { # [cfg (not (any (target_env = "sgx" , target_os = "none" , target_os = "uefi")))] $ body # [cfg (any (target_env = "sgx" , target_os = "none" , target_os = "uefi"))] false } # [cfg (all ($ (target_feature =$ tf ,) *))] true } } ; }
};
}
