// Generated macro for macro_36 (macro)
macro_rules! Depcratemacro_36 {
() => {
// Module: crate
// Provides: {"macro_36"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (any (all (any (target_arch = "x86" , target_arch = "x86_64") , target_feature = "aes" , not (miri)) , all (target_arch = "aarch64" , target_feature = "aes" , not (miri)) , all (feature = "nightly-arm-aes" , target_arch = "arm" , target_feature = "aes" , not (miri)) ,))] { mod aes_hash ; pub use crate :: aes_hash :: AHasher ; } else { pub use crate :: fallback_hash :: AHasher ; } }
};
}
