// Generated macro for is_enabled (function)
macro_rules! Depcrate_transforms_threadsis_enabled {
() => {
// Module: crate::transforms::threads
// Provides: {"is_enabled"}
// Dependencies: {}
# [doc = " Is threaded Wasm enabled?"] pub fn is_enabled (module : & Module) -> bool { match wasm_conventions :: get_memory (module) { Ok (memory) => module . memories . get (memory) . shared , Err (_) => false , } }
};
}
