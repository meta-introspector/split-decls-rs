// Generated macro for prop_reverse_reverse (function)
macro_rules! Depcrate_testsprop_reverse_reverse {
() => {
// Module: crate::tests
// Provides: {"prop_reverse_reverse"}
// Dependencies: {}
# [test] fn prop_reverse_reverse () { fn prop (xs : Vec < usize >) -> bool { let rev : Vec < _ > = xs . clone () . into_iter () . rev () . collect () ; let revrev : Vec < _ > = rev . into_iter () . rev () . collect () ; xs == revrev } quickcheck (prop as fn (Vec < usize >) -> bool) ; }
};
}
