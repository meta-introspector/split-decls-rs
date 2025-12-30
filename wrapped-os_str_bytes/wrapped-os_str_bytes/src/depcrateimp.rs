// Generated macro for imp (module)
macro_rules! Depcrateimp {
() => {
// Module: crate
// Provides: {"imp"}
// Dependencies: {}
# [cfg_attr (all (target_family = "wasm" , target_os = "unknown") , path = "wasm/mod.rs")] # [cfg_attr (any (target_os = "uefi" , windows) , path = "windows/mod.rs")] # [cfg_attr (not (any (all (target_family = "wasm" , target_os = "unknown") , target_os = "uefi" , windows ,)) , path = "common/mod.rs")] mod imp ;
};
}
