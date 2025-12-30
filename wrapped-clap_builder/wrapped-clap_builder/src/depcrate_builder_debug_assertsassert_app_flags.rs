// Generated macro for assert_app_flags (function)
macro_rules! Depcrate_builder_debug_assertsassert_app_flags {
() => {
// Module: crate::builder::debug_asserts
// Provides: {"assert_app_flags"}
// Dependencies: {}
fn assert_app_flags (cmd : & Command) { macro_rules ! checker { ($ a : ident conflicts $ ($ b : ident) |+) => { if cmd .$ a () { let mut s = String :: new () ; $ (if cmd .$ b () { use std :: fmt :: Write ; write ! (& mut s , "  AppSettings::{} conflicts with AppSettings::{}.\n" , std :: stringify ! ($ b) , std :: stringify ! ($ a)) . unwrap () ; }) + if ! s . is_empty () { panic ! ("{}\n{}" , cmd . get_name () , s) } } } ; } checker ! (is_multicall_set conflicts is_no_binary_name_set) ; }
};
}
