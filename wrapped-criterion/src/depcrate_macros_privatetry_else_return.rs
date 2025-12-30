// Generated macro for try_else_return (macro)
macro_rules! Depcrate_macros_privatetry_else_return {
() => {
// Module: crate::macros_private
// Provides: {"try_else_return"}
// Dependencies: {}
# [doc = " Matches a result, returning the `Ok` value in case of success,"] # [doc = " exits the calling function otherwise."] # [doc = " A closure which returns the return value for the function can"] # [doc = " be passed as second parameter."] macro_rules ! try_else_return { ($ x : expr) => { try_else_return ! ($ x , || { }) } ; ($ x : expr , $ el : expr) => { match $ x { Ok (x) => x , Err (e) => { crate :: error :: log_error (& e) ; let closure = $ el ; return closure () ; } } } ; }
};
}
