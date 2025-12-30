// Generated macro for call_node_node_type_n_times (function)
macro_rules! Depcratecall_node_node_type_n_times {
() => {
// Module: crate
// Provides: {"call_node_node_type_n_times"}
// Dependencies: {}
# [wasm_bindgen] pub fn call_node_node_type_n_times (n : usize , elements : Vec < JsValue >) { for _ in 0 .. n { for element in elements . iter () { let element = element . unchecked_ref :: < Node > () ; assert ! (element . node_type () != 100) ; } } }
};
}
