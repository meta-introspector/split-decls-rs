// Generated macro for get_argument_tokens (function)
macro_rules! Depcrate_unexposed_attrget_argument_tokens {
() => {
// Module: crate::unexposed_attr
// Provides: {"get_argument_tokens"}
// Dependencies: {}
pub (crate) fn get_argument_tokens < 'a > (entity : & Entity < 'a >) -> Vec < Token < 'a > > { if ! entity . is_function_like_macro () { return vec ! [] ; } let name_ranges = entity . get_name_ranges () ; assert_eq ! (name_ranges . len () , 1 , "macro name ranges") ; let name_range = name_ranges . first () . unwrap () ; let range = entity . get_range () . expect ("macro range") ; if range . get_start () == range . get_end () { return vec ! [] ; } SourceRange :: new (name_range . get_end () , range . get_end ()) . tokenize () }
};
}
