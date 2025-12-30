// Generated macro for to_snake_case (function)
macro_rules! Depcrateto_snake_case {
() => {
// Module: crate
// Provides: {"to_snake_case"}
// Dependencies: {}
pub (crate) fn to_snake_case (input : impl AsRef < str >) -> String { let input = input . as_ref () ; if input == "_" { String :: from ("_") } else { heck :: ToSnakeCase :: to_snake_case (input) } }
};
}
