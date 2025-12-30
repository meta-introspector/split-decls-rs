// Generated macro for imp (module)
macro_rules! Depcrateimp {
() => {
// Module: crate
// Provides: {"imp"}
// Dependencies: {}
# [cfg_attr (any (target_os = "linux" , target_os = "android") , path = "linux.rs")] # [cfg_attr (target_os = "freebsd" , path = "freebsd.rs")] # [cfg_attr (any (target_os = "macos" , target_os = "ios") , path = "apple.rs")] # [cfg_attr (target_os = "aix" , path = "aix.rs")] mod imp ;
};
}
