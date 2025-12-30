// Generated macro for match_by_ident (function)
macro_rules! Depcrate_macro_expansion_tests_mbematch_by_ident {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"match_by_ident"}
// Dependencies: {}
# [test] fn match_by_ident () { check (r#"
macro_rules! m {
    ($i:ident) => ( mod $i {} );
    (spam $i:ident) => ( fn $i() {} );
    (eggs $i:ident) => ( struct $i; )
}
m! { foo }
m! { spam bar }
m! { eggs Baz }
"# , expect ! [[r#"
macro_rules! m {
    ($i:ident) => ( mod $i {} );
    (spam $i:ident) => ( fn $i() {} );
    (eggs $i:ident) => ( struct $i; )
}
mod foo {}
fn bar() {}
struct Baz;
"#]] ,) ; }
};
}
