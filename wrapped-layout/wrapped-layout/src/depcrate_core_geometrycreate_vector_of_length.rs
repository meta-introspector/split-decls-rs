// Generated macro for create_vector_of_length (function)
macro_rules! Depcrate_core_geometrycreate_vector_of_length {
() => {
// Module: crate::core::geometry
// Provides: {"create_vector_of_length"}
// Dependencies: {}
pub fn create_vector_of_length (from : Point , to : Point , s : f64 ,) -> (Point , Point) { if from == to { return (from , Point :: new (from . x + s , from . y)) ; } let t = to . sub (from) ; let t = normalize_scale_vector (t , s) ; (from , t . add (from)) }
};
}
