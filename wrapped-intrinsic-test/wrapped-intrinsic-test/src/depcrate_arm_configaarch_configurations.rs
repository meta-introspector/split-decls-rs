// Generated macro for AARCH_CONFIGURATIONS (const)
macro_rules! Depcrate_arm_configAARCH_CONFIGURATIONS {
() => {
// Module: crate::arm::config
// Provides: {"AARCH_CONFIGURATIONS"}
// Dependencies: {}
pub const AARCH_CONFIGURATIONS : & str = r#"
#![cfg_attr(target_arch = "arm", feature(stdarch_arm_neon_intrinsics))]
#![cfg_attr(target_arch = "arm", feature(stdarch_aarch32_crc32))]
#![cfg_attr(any(target_arch = "aarch64", target_arch = "arm64ec"), feature(stdarch_neon_fcma))]
#![cfg_attr(any(target_arch = "aarch64", target_arch = "arm64ec"), feature(stdarch_neon_dotprod))]
#![cfg_attr(any(target_arch = "aarch64", target_arch = "arm64ec"), feature(stdarch_neon_i8mm))]
#![cfg_attr(any(target_arch = "aarch64", target_arch = "arm64ec"), feature(stdarch_neon_sm4))]
#![cfg_attr(any(target_arch = "aarch64", target_arch = "arm64ec"), feature(stdarch_neon_ftts))]
#![feature(fmt_helpers_for_derive)]
#![feature(stdarch_neon_f16)]
"# ;
};
}
