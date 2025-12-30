// Generated macro for type_suitable_to_unwrap (function)
macro_rules! Depcrate_assertions_on_result_statestype_suitable_to_unwrap {
() => {
// Module: crate::assertions_on_result_states
// Provides: {"type_suitable_to_unwrap"}
// Dependencies: {}
fn type_suitable_to_unwrap < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { has_debug_impl (cx , ty) && ! ty . is_unit () && ! ty . is_never () }
};
}
