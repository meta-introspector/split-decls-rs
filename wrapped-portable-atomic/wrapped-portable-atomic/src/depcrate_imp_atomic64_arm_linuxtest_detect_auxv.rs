// Generated macro for test_detect_auxv (module)
macro_rules! Depcrate_imp_atomic64_arm_linuxtest_detect_auxv {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"test_detect_auxv"}
// Dependencies: {}
# [cfg (test)] # [cfg (not (portable_atomic_no_outline_atomics))] # [cfg (any (all (target_os = "linux" , any (target_env = "gnu" , target_env = "musl" , target_env = "ohos" , all (target_env = "uclibc" , not (target_feature = "crt-static")) ,) ,) , target_os = "android" , target_os = "freebsd" , target_os = "openbsd" ,))] # [path = "../detect/auxv.rs"] mod test_detect_auxv ;
};
}
