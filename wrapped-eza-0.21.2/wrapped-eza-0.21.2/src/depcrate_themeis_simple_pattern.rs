// Generated macro for is_simple_pattern (function)
macro_rules! Depcrate_themeis_simple_pattern {
() => {
// Module: crate::theme
// Provides: {"is_simple_pattern"}
// Dependencies: {}
fn is_simple_pattern (pattern : glob :: Pattern) -> Result < String , glob :: Pattern > { match pattern . as_str () . strip_prefix ("*.") { None => Err (pattern) , Some (ext) if ext . contains (['?' , '*' , '[' , ']' , '.']) => Err (pattern) , Some (ext) => Ok (ext . to_string ()) , } }
};
}
