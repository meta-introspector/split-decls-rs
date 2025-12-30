// Generated macro for transform_test_function (function)
macro_rules! Depcrate_arbitrary_in_macro_rulestransform_test_function {
() => {
// Module: crate::arbitrary_in_macro_rules
// Provides: {"transform_test_function"}
// Dependencies: {}
fn transform_test_function (fn_item : & mut ItemFn) { let mut modified_stmts = Vec :: new () ; let mut has_types = false ; for stmt in & fn_item . block . stmts { match stmt { Stmt :: Item (item @ (Item :: Struct (_) | Item :: Enum (_))) => { if has_derive_arbitrary_attr (item . attrs ()) { has_types = true ; let traits = extract_derive_traits (item . attrs ()) ; let macro_name = generate_macro_name (& traits) ; let mut cleaned_item = item . clone () ; cleaned_item . attrs_mut () . retain (| attr | ! attr . path () . is_ident ("derive")) ; let macro_ident = syn :: Ident :: new (& macro_name , proc_macro2 :: Span :: call_site ()) ; let macro_call : Stmt = parse_quote ! { # macro_ident ! { # cleaned_item } ; } ; modified_stmts . push (macro_call) ; } else { modified_stmts . push (stmt . clone ()) ; } } _ => { modified_stmts . push (stmt . clone ()) ; } } } if has_types { fn_item . block . stmts = modified_stmts ; } }
};
}
