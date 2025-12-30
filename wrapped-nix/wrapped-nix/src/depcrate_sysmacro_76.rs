// Generated macro for macro_76 (macro)
macro_rules! Depcrate_sysmacro_76 {
() => {
// Module: crate::sys
// Provides: {"macro_76"}
// Dependencies: {}
# [cfg (any (freebsdlike , all (target_os = "linux" , not (any (target_env = "uclibc" , target_env = "ohos"))) , apple_targets , target_os = "netbsd"))] feature ! { #! [feature = "aio"] pub mod aio ; }
};
}
