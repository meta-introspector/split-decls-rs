// Generated macro for macro_60 (macro)
macro_rules! Depcrate_thread_parkermacro_60 {
() => {
// Module: crate::thread_parker
// Provides: {"macro_60"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (target_os = "linux" , target_os = "android"))] { # [path = "linux.rs"] mod imp ; } else if # [cfg (unix)] { # [path = "unix.rs"] mod imp ; } else if # [cfg (windows)] { # [path = "windows/mod.rs"] mod imp ; } else if # [cfg (target_os = "redox")] { # [path = "redox.rs"] mod imp ; } else if # [cfg (all (target_env = "sgx" , target_vendor = "fortanix"))] { # [path = "sgx.rs"] mod imp ; } else if # [cfg (all (feature = "nightly" , target_family = "wasm" , target_feature = "atomics"))] { # [path = "wasm_atomic.rs"] mod imp ; } else if # [cfg (target_family = "wasm")] { # [path = "wasm.rs"] mod imp ; } else { # [path = "generic.rs"] mod imp ; } }
};
}
