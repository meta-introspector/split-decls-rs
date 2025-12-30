// Generated macro for call_first_child_final_n_times (function)
macro_rules! Depcratecall_first_child_final_n_times {
() => {
// Module: crate
// Provides: {"call_first_child_final_n_times"}
// Dependencies: {}
# [wasm_bindgen] pub fn call_first_child_final_n_times (n : usize , element : & Element) { for _ in 0 .. n { drop (element . first_child_final ()) ; } }
};
}
