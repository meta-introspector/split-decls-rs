// Generated macro for get_graph_order_as_bits (function)
macro_rules! Depcrate_graph6_graph6_encoderget_graph_order_as_bits {
() => {
// Module: crate::graph6::graph6_encoder
// Provides: {"get_graph_order_as_bits"}
// Dependencies: {}
fn get_graph_order_as_bits (order : usize) -> Vec < usize > { let to_convert_to_bits = if order < N { vec ! [(order , 6)] } else if order <= 258047 { vec ! [(N , 6) , (order , 18)] } else { panic ! ("Graph order not supported.") } ; to_convert_to_bits . iter () . flat_map (| & (n , n_of_bits) | get_number_as_bits (n , n_of_bits)) . collect () }
};
}
