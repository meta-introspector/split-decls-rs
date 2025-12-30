// Generated macro for wasm (module)
macro_rules! Depcratewasm {
() => {
// Module: crate
// Provides: {"wasm"}
// Dependencies: {}
# [cfg (any (target_os = "wasi" , target_family = "wasm"))] mod wasm { use std :: path :: Path ; use super :: IsExecutable ; impl IsExecutable for Path { fn is_executable (& self) -> bool { false } } }
};
}
