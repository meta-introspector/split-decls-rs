// Generated macro for eager_expands_with_unresolved_within (function)
macro_rules! Depcrate_macro_expansion_tests_mbeeager_expands_with_unresolved_within {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"eager_expands_with_unresolved_within"}
// Dependencies: {}
# [test] fn eager_expands_with_unresolved_within () { check (r#"
#[rustc_builtin_macro]
#[macro_export]
macro_rules! concat {}
macro_rules! identity {
    ($tt:tt) => {
        $tt
    }
}

fn main(foo: ()) {
    concat!("hello", identity!("world"), unresolved!(), identity!("!"));
}
"# , expect ! [[r##"
#[rustc_builtin_macro]
#[macro_export]
macro_rules! concat {}
macro_rules! identity {
    ($tt:tt) => {
        $tt
    }
}

fn main(foo: ()) {
    /* error: unresolved macro unresolved */"helloworld!";
}
"##]] ,) ; }
};
}
