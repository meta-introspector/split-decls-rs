// Generated macro for wants_wasm_eh (function)
macro_rules! Depcrate_basewants_wasm_eh {
() => {
// Module: crate::base
// Provides: {"wants_wasm_eh"}
// Dependencies: {}
pub fn wants_wasm_eh (sess : & Session) -> bool { sess . target . is_like_wasm && (sess . target . os != "emscripten" || sess . opts . unstable_opts . emscripten_wasm_eh) }
};
}
