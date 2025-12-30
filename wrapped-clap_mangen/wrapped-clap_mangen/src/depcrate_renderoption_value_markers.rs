// Generated macro for option_value_markers (function)
macro_rules! Depcrate_renderoption_value_markers {
() => {
// Module: crate::render
// Provides: {"option_value_markers"}
// Dependencies: {}
fn option_value_markers (arg : & Arg) -> (& 'static str , & 'static str) { let range = arg . get_num_args () . expect ("built") ; if ! range . takes_values () { return ("" , "") ; } let required = range . min_values () > 0 ; let require_equals = arg . is_require_equals_set () ; match (required , require_equals) { (true , false) => (" " , "") , (false , false) => (" [" , "]") , (false , true) => ("[=" , "]") , (true , true) => ("=" , "") , } }
};
}
