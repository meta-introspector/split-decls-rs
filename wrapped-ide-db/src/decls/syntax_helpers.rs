macro_rules! syntax_helpers {
    () => {
        pub mod syntax_helpers { pub mod format_string ; pub mod format_string_exprs ; pub mod tree_diff ; pub use hir :: prettify_macro_expansion ; pub mod node_ext ; pub mod suggest_name ; pub use parser :: LexedStr ; }
    };
}

syntax_helpers!()