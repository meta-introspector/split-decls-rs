// Generated macro for error_node_position (macro)
macro_rules! Depcrate_errorerror_node_position {
() => {
// Module: crate::error
// Provides: {"error_node_position"}
// Dependencies: {}
# [doc = " Creates a parse error from a `nom::ErrorKind`,"] # [doc = " the position in the input and the next error in"] # [doc = " the parsing tree"] # [allow (unused_variables)] # [macro_export (local_inner_macros)] macro_rules ! error_node_position (($ input : expr , $ code : expr , $ next : expr $ (,) ?) => ({ $ crate :: error :: append_error ($ input , $ code , $ next) }) ;) ;
};
}
