// Generated macro for assert_arg_flags (function)
macro_rules! Depcrate_builder_debug_assertsassert_arg_flags {
() => {
// Module: crate::builder::debug_asserts
// Provides: {"assert_arg_flags"}
// Dependencies: {}
fn assert_arg_flags (arg : & Arg) { macro_rules ! checker { ($ a : ident requires $ ($ b : ident) |+) => { if arg .$ a () { let mut s = String :: new () ; $ (if ! arg .$ b () { use std :: fmt :: Write ; write ! (& mut s , "  Arg::{} is required when Arg::{} is set.\n" , std :: stringify ! ($ b) , std :: stringify ! ($ a)) . unwrap () ; }) + if ! s . is_empty () { panic ! ("Argument {:?}\n{}" , arg . get_id () , s) } } } } checker ! (is_hide_possible_values_set requires is_takes_value_set) ; checker ! (is_allow_hyphen_values_set requires is_takes_value_set) ; checker ! (is_allow_negative_numbers_set requires is_takes_value_set) ; checker ! (is_require_equals_set requires is_takes_value_set) ; checker ! (is_last_set requires is_takes_value_set) ; checker ! (is_hide_default_value_set requires is_takes_value_set) ; checker ! (is_multiple_values_set requires is_takes_value_set) ; checker ! (is_ignore_case_set requires is_takes_value_set) ; }
};
}
