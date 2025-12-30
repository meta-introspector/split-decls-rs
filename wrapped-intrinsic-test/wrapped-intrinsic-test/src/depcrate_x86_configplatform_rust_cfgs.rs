// Generated macro for PLATFORM_RUST_CFGS (const)
macro_rules! Depcrate_x86_configPLATFORM_RUST_CFGS {
() => {
// Module: crate::x86::config
// Provides: {"PLATFORM_RUST_CFGS"}
// Dependencies: {}
pub const PLATFORM_RUST_CFGS : & str = r#"
#![cfg_attr(target_arch = "x86", feature(avx))]
#![cfg_attr(target_arch = "x86", feature(sse))]
#![cfg_attr(target_arch = "x86", feature(sse2))]
#![cfg_attr(target_arch = "x86", feature(stdarch_x86_avx512_bf16))]
#![cfg_attr(target_arch = "x86", feature(stdarch_x86_avx512_f16))]
#![cfg_attr(target_arch = "x86", feature(stdarch_x86_rtm))]
#![cfg_attr(target_arch = "x86", feature(stdarch_x86_rtm))]
#![cfg_attr(target_arch = "x86_64", feature(x86_amx_intrinsics))]
#![cfg_attr(target_arch = "x86_64", feature(stdarch_x86_avx512_f16))]
#![feature(fmt_helpers_for_derive)]
"# ;
};
}
