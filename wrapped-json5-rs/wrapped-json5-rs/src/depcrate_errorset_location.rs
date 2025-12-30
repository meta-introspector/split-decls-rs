// Generated macro for set_location (function)
macro_rules! Depcrate_errorset_location {
() => {
// Module: crate::error
// Provides: {"set_location"}
// Dependencies: {}
# [doc = " Adds location information from `span`, if `res` is an error."] pub fn set_location < T > (res : & mut Result < T > , span : & Span < '_ >) { if let Err (ref mut e) = res { let Error :: Message { location , .. } = e ; if location . is_none () { let (line , column) = span . start_pos () . line_col () ; * location = Some (Location { line , column }) ; } } }
};
}
