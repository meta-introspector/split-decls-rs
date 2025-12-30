// Generated macro for macro_6 (macro)
macro_rules! Depcratemacro_6 {
() => {
// Module: crate
// Provides: {"macro_6"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (target_os = "redox")] { # [path = "redox.rs"] mod imp ; } else if # [cfg (windows)] { # [path = "windows.rs"] mod imp ; } else if # [cfg (all (target_family = "wasm" , not (target_os = "emscripten")))] { # [path = "wasm.rs"] mod imp ; } else { # [path = "unix/mod.rs"] mod imp ; } }
};
}
