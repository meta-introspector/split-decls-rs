// Generated macro for match_by_separator_token (function)
macro_rules! Depcrate_macro_expansion_tests_mbematch_by_separator_token {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"match_by_separator_token"}
// Dependencies: {}
# [test] fn match_by_separator_token () { check (r#"
macro_rules! m {
    ($($i:ident),*) => ($(mod $i {} )*);
    ($($i:ident)#*) => ($(fn $i() {} )*);
    ($i:ident ,# $ j:ident) => ( struct $i; struct $ j; )
}

m! { foo, bar }

m! { foo# bar }

m! { Foo,# Bar }
"# , expect ! [[r#"
macro_rules! m {
    ($($i:ident),*) => ($(mod $i {} )*);
    ($($i:ident)#*) => ($(fn $i() {} )*);
    ($i:ident ,# $ j:ident) => ( struct $i; struct $ j; )
}

mod foo {}
mod bar {}

fn foo() {}
fn bar() {}

struct Foo;
struct Bar;
"#]] ,) ; }
};
}
