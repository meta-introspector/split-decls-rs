// Generated macro for validate_left_recursion (function)
macro_rules! Depcrate_validatorvalidate_left_recursion {
() => {
// Module: crate::validator
// Provides: {"validate_left_recursion"}
// Dependencies: {}
fn validate_left_recursion < 'a , 'i : 'a > (rules : & 'a [ParserRule < 'i >]) -> Vec < Error < Rule > > { left_recursion (to_hash_map (rules)) }
};
}
