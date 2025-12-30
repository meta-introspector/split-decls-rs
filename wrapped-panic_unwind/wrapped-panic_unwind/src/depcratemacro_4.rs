// Generated macro for macro_4 (macro)
macro_rules! Depcratemacro_4 {
() => {
// Module: crate
// Provides: {"macro_4"}
// Dependencies: {}
cfg_select ! { all (target_os = "emscripten" , not (emscripten_wasm_eh)) => { # [path = "emcc.rs"] mod imp ; } target_os = "hermit" => { # [path = "hermit.rs"] mod imp ; } target_os = "l4re" => { # [path = "dummy.rs"] mod imp ; } any (all (target_family = "windows" , target_env = "gnu") , target_os = "psp" , target_os = "xous" , target_os = "solid_asp3" , all (target_family = "unix" , not (any (target_os = "espidf" , target_os = "nuttx"))) , all (target_vendor = "fortanix" , target_env = "sgx") , target_family = "wasm" ,) => { # [path = "gcc.rs"] mod imp ; } miri => { # [path = "miri.rs"] mod imp ; } all (target_env = "msvc" , not (target_arch = "arm")) => { # [path = "seh.rs"] mod imp ; } _ => { # [path = "dummy.rs"] mod imp ; } }
};
}
