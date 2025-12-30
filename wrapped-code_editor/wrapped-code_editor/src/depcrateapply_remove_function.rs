// Generated macro for apply_remove_function (function)
macro_rules! Depcrateapply_remove_function {
() => {
// Module: crate
// Provides: {"apply_remove_function"}
// Dependencies: {}
fn apply_remove_function (ast : & mut File , details : & RemoveFunctionDetails) -> Result < () > { let mut removed = false ; ast . items . retain (| item | { if let Item :: Fn (item_fn) = item { if item_fn . sig . ident == details . function_name { removed = true ; return false ; } } true }) ; if removed { println ! ("  Removed function: {}" , details . function_name) ; } else { println ! ("  Warning: Function '{}' not found for removal." , details . function_name) ; } Ok (()) }
};
}
