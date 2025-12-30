// Generated macro for reverse_app (function)
macro_rules! Depcrate_testsreverse_app {
() => {
// Module: crate::tests
// Provides: {"reverse_app"}
// Dependencies: {}
# [test] fn reverse_app () { fn prop (xs : Vec < usize > , ys : Vec < usize >) -> bool { let mut app = xs . clone () ; app . extend (ys . iter () . copied ()) ; let app_rev : Vec < usize > = app . into_iter () . rev () . collect () ; let rxs : Vec < usize > = xs . into_iter () . rev () . collect () ; let mut rev_app = ys . into_iter () . rev () . collect :: < Vec < usize > > () ; rev_app . extend (rxs) ; app_rev == rev_app } quickcheck (prop as fn (Vec < usize > , Vec < usize >) -> bool) ; }
};
}
