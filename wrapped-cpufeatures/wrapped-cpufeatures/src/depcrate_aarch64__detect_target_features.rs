// Generated macro for __detect_target_features (macro)
macro_rules! Depcrate_aarch64__detect_target_features {
() => {
// Module: crate::aarch64
// Provides: {"__detect_target_features"}
// Dependencies: {}
# [cfg (not (any (target_vendor = "apple" , target_os = "linux" , target_os = "android" ,)))] # [macro_export] # [doc (hidden)] macro_rules ! __detect_target_features { ($ ($ tf : tt) ,+) => { false } ; }
};
}
