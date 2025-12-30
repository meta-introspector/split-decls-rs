// Generated macro for assert_debug_eq (function)
macro_rules! Depcrateassert_debug_eq {
() => {
// Module: crate
// Provides: {"assert_debug_eq"}
// Dependencies: {}
pub fn assert_debug_eq (a : impl std :: fmt :: Debug , e : impl std :: fmt :: Debug) { assert_eq ! (format ! ("{a:?}") , format ! ("{e:?}")) ; assert_eq ! (format ! ("{a:#?}") , format ! ("{e:#?}")) ; }
};
}
