// Generated macro for call_doesnt_throw_with_catch_n_times (function)
macro_rules! Depcratecall_doesnt_throw_with_catch_n_times {
() => {
// Module: crate
// Provides: {"call_doesnt_throw_with_catch_n_times"}
// Dependencies: {}
# [wasm_bindgen] pub fn call_doesnt_throw_with_catch_n_times (n : usize) { for _ in 0 .. n { if let Err (e) = doesnt_throw_catch () { wasm_bindgen :: throw_val (e) ; } } }
};
}
