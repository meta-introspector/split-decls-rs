// Generated macro for mbe_smoke_test (function)
macro_rules! Depcrate_macro_expansion_tests_mbembe_smoke_test {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"mbe_smoke_test"}
// Dependencies: {}
# [test] fn mbe_smoke_test () { check (r#"
macro_rules! impl_froms {
    ($e:ident: $($v:ident),*) => {
        $(
            impl From<$v> for $e {
                fn from(it: $v) -> $e { $e::$v(it) }
            }
        )*
    }
}
impl_froms!(TokenTree: Leaf, Subtree);
"# , expect ! [[r#"
macro_rules! impl_froms {
    ($e:ident: $($v:ident),*) => {
        $(
            impl From<$v> for $e {
                fn from(it: $v) -> $e { $e::$v(it) }
            }
        )*
    }
}
impl From<Leaf> for TokenTree {
    fn from(it: Leaf) -> TokenTree {
        TokenTree::Leaf(it)
    }
}
impl From<Subtree> for TokenTree {
    fn from(it: Subtree) -> TokenTree {
        TokenTree::Subtree(it)
    }
}
"#]] ,) ; }
};
}
