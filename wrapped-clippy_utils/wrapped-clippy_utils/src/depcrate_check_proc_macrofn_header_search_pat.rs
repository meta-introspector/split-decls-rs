// Generated macro for fn_header_search_pat (function)
macro_rules! Depcrate_check_proc_macrofn_header_search_pat {
() => {
// Module: crate::check_proc_macro
// Provides: {"fn_header_search_pat"}
// Dependencies: {}
fn fn_header_search_pat (header : FnHeader) -> Pat { if header . is_async () { Pat :: Str ("async") } else if header . is_const () { Pat :: Str ("const") } else if header . is_unsafe () { Pat :: Str ("unsafe") } else if header . abi != ExternAbi :: Rust { Pat :: Str ("extern") } else { Pat :: MultiStr (& ["fn" , "extern"]) } }
};
}
