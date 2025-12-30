// Generated macro for assert_empty (function)
macro_rules! Depcrate_all_supportassert_empty {
() => {
// Module: crate::all::support
// Provides: {"assert_empty"}
// Dependencies: {}
pub fn assert_empty < T : Future , F : FnMut () -> T > (mut f : F) { assert ! (f () . poll (& mut Task :: new ()) . is_not_ready ()) ; let mut a = f () ; let mut task = Task :: new () ; a . schedule (& mut task) ; assert ! (a . poll (& mut task) . is_not_ready ()) ; drop (a) ; }
};
}
