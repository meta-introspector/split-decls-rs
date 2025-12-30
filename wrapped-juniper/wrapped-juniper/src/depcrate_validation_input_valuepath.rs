// Generated macro for Path (enum)
macro_rules! Depcrate_validation_input_valuePath {
() => {
// Module: crate::validation::input_value
// Provides: {"Path"}
// Dependencies: {}
# [derive (Debug , Display)] enum Path < 'a > { # [display ("")] Root , # [display ("{_1}In element #{_0}: ")] ArrayElement (usize , & 'a Path < 'a >) , # [display (r#"{_1}In field "{_0}": "#)] ObjectField (& 'a str , & 'a Path < 'a >) , }
};
}
