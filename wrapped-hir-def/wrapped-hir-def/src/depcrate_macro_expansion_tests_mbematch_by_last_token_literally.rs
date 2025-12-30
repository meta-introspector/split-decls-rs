// Generated macro for match_by_last_token_literally (function)
macro_rules! Depcrate_macro_expansion_tests_mbematch_by_last_token_literally {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"match_by_last_token_literally"}
// Dependencies: {}
# [test] fn match_by_last_token_literally () { check (r#"
macro_rules! m {
    ($i:ident) => ( mod $i {} );
    ($i:ident =) => ( fn $i() {} );
    ($i:ident +) => ( struct $i; )
}
m! { foo }
m! { bar = }
m! { Baz + }
"# , expect ! [[r#"
macro_rules! m {
    ($i:ident) => ( mod $i {} );
    ($i:ident =) => ( fn $i() {} );
    ($i:ident +) => ( struct $i; )
}
mod foo {}
fn bar() {}
struct Baz;
"#]] ,) ; }
};
}
