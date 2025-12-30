// Generated macro for visualize_whitespace (function)
macro_rules! Depcrate_errorvisualize_whitespace {
() => {
// Module: crate::error
// Provides: {"visualize_whitespace"}
// Dependencies: {}
fn visualize_whitespace (input : & str) -> String { input . to_owned () . replace ('\r' , "␍") . replace ('\n' , "␊") }
};
}
