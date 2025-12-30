// Generated macro for lifetime_repeat (function)
macro_rules! Depcrate_macro_expansion_tests_mbelifetime_repeat {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"lifetime_repeat"}
// Dependencies: {}
# [test] fn lifetime_repeat () { check (r#"
macro_rules! m {
    ($($x:expr)'a*) => (stringify!($($x)'b*));
}
fn f() {
    let _ = m!(0 'a 1 'a 2);
}
    "# , expect ! [[r#"
macro_rules! m {
    ($($x:expr)'a*) => (stringify!($($x)'b*));
}
fn f() {
    let _ = stringify!(0 'b 1 'b 2);
}
    "#]] ,) ; }
};
}
