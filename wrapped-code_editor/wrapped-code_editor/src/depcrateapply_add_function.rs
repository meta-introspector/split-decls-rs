// Generated macro for apply_add_function (function)
macro_rules! Depcrateapply_add_function {
() => {
// Module: crate
// Provides: {"apply_add_function"}
// Dependencies: {}
fn apply_add_function (ast : & mut File , details : & AddFunctionDetails) -> Result < () > { let new_fn : ItemFn = syn :: parse_str (& details . function_code) . with_context (| | format ! ("Invalid function code syntax: {}" , details . function_code)) ? ; let new_fn_name = new_fn . sig . ident . to_string () ; for item in & ast . items { if let Item :: Fn (existing_fn) = item { if existing_fn . sig . ident . to_string () == new_fn_name { println ! ("  Warning: Function '{}' already exists. Skipping addition." , new_fn_name) ; return Ok (()) } } } ast . items . push (Item :: Fn (new_fn)) ; println ! ("  Added function: {}" , new_fn_name) ; Ok (()) }
};
}
