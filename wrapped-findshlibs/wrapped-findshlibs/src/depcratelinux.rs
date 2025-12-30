// Generated macro for linux (module)
macro_rules! Depcratelinux {
() => {
// Module: crate
// Provides: {"linux"}
// Dependencies: {}
# [cfg (any (target_os = "linux" , all (target_os = "android" , feature = "dl_iterate_phdr")))] pub mod linux ;
};
}
