// Generated macro for macro_82 (macro)
macro_rules! Depcrate_random_statemacro_82 {
() => {
// Module: crate::random_state
// Provides: {"macro_82"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (any (all (any (target_arch = "x86" , target_arch = "x86_64") , target_feature = "aes" , not (miri)) , all (target_arch = "aarch64" , target_feature = "aes" , not (miri)) , all (feature = "nightly-arm-aes" , target_arch = "arm" , target_feature = "aes" , not (miri)) ,))] { use crate :: aes_hash ::*; } else { use crate :: fallback_hash ::*; } }
};
}
