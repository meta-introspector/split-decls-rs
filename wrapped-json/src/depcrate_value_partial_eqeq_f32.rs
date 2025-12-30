// Generated macro for eq_f32 (function)
macro_rules! Depcrate_value_partial_eqeq_f32 {
() => {
// Module: crate::value::partial_eq
// Provides: {"eq_f32"}
// Dependencies: {}
fn eq_f32 (value : & Value , other : f32) -> bool { match value { Value :: Number (n) => n . as_f32 () == Some (other) , _ => false , } }
};
}
