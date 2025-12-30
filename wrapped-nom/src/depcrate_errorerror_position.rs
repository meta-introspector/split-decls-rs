// Generated macro for error_position (macro)
macro_rules! Depcrate_errorerror_position {
() => {
// Module: crate::error
// Provides: {"error_position"}
// Dependencies: {}
# [doc = " Creates a parse error from a `nom::ErrorKind`"] # [doc = " and the position in the input"] # [allow (unused_variables)] # [macro_export (local_inner_macros)] macro_rules ! error_position (($ input : expr , $ code : expr $ (,) ?) => ({ $ crate :: error :: make_error ($ input , $ code) }) ;) ;
};
}
