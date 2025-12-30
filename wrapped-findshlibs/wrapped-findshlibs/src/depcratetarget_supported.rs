// Generated macro for TARGET_SUPPORTED (const)
macro_rules! DepcrateTARGET_SUPPORTED {
() => {
// Module: crate
// Provides: {"TARGET_SUPPORTED"}
// Dependencies: {}
# [doc = " An indicator if this platform is supported."] pub const TARGET_SUPPORTED : bool = cfg ! (any (target_os = "macos" , target_os = "ios" , target_os = "linux" , all (target_os = "android" , feature = "dl_iterate_phdr") , target_os = "windows")) ;
};
}
