// Generated macro for macro_223 (macro)
macro_rules! Depcrate_cpumacro_223 {
() => {
// Module: crate::cpu
// Provides: {"macro_223"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (target_arch = "aarch64")] { pub mod aarch64 ; use aarch64 :: featureflags ; } else if # [cfg (target_arch = "arm")] { pub mod arm ; use arm :: featureflags ; } else if # [cfg (target_arch = "x86")] { pub mod x86 ; use x86 :: featureflags ; pub use x86 as intel ; } else if # [cfg (target_arch = "x86_64")] { pub mod x86_64 ; use x86_64 :: featureflags ; pub use x86_64 as intel ; } else { mod featureflags { use super :: Features ; # [inline (always)] pub (super) fn get_or_init () -> Features { Features :: new_no_features_to_detect () } # [inline (always)] pub (super) fn get (_cpu_features : Features) -> u32 { STATIC_DETECTED } pub (super) const STATIC_DETECTED : u32 = 0 ; pub (super) const FORCE_DYNAMIC_DETECTION : u32 = 0 ; } } }
};
}
