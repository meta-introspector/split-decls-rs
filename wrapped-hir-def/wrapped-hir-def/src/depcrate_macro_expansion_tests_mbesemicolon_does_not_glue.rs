// Generated macro for semicolon_does_not_glue (function)
macro_rules! Depcrate_macro_expansion_tests_mbesemicolon_does_not_glue {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"semicolon_does_not_glue"}
// Dependencies: {}
# [test] fn semicolon_does_not_glue () { check (r#"
macro_rules! bug {
    ($id: expr) => {
        true
    };
    ($id: expr; $($attr: ident),*) => {
        true
    };
    ($id: expr; $($attr: ident),*; $norm: expr) => {
        true
    };
    ($id: expr; $($attr: ident),*;; $print: expr) => {
        true
    };
    ($id: expr; $($attr: ident),*; $norm: expr; $print: expr) => {
        true
    };
}
fn f() {
    let _ = bug!(a;;;test);
}
    "# , expect ! [[r#"
macro_rules! bug {
    ($id: expr) => {
        true
    };
    ($id: expr; $($attr: ident),*) => {
        true
    };
    ($id: expr; $($attr: ident),*; $norm: expr) => {
        true
    };
    ($id: expr; $($attr: ident),*;; $print: expr) => {
        true
    };
    ($id: expr; $($attr: ident),*; $norm: expr; $print: expr) => {
        true
    };
}
fn f() {
    let _ = true;
}
    "#]] ,) ; }
};
}
