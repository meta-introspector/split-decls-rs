// Generated macro for count_node_types (function)
macro_rules! Depcratecount_node_types {
() => {
// Module: crate
// Provides: {"count_node_types"}
// Dependencies: {}
# [wasm_bindgen] pub fn count_node_types (element : Node) { let mut count = Vec :: new () ; count_node_types (element , & mut count) ; fn count_node_types (mut element : Node , count : & mut Vec < u32 >) { loop { let t = element . node_type () ; if t as usize >= count . len () { count . resize (t as usize + 1 , 0) ; } count [t as usize] += 1 ; if let Some (s) = element . first_child () { count_node_types (s , count) ; } match element . next_sibling () { Some (s) => element = s , None => break , } } } }
};
}
