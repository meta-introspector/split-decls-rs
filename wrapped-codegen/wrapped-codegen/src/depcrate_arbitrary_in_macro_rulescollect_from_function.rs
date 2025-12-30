// Generated macro for collect_from_function (function)
macro_rules! Depcrate_arbitrary_in_macro_rulescollect_from_function {
() => {
// Module: crate::arbitrary_in_macro_rules
// Provides: {"collect_from_function"}
// Dependencies: {}
fn collect_from_function (fn_item : & ItemFn , combinations : & mut std :: collections :: BTreeSet < Vec < String > > ,) { for stmt in & fn_item . block . stmts { if let Stmt :: Item (item @ (Item :: Struct (_) | Item :: Enum (_))) = stmt { if has_derive_arbitrary_attr (item . attrs ()) { let traits = extract_derive_traits (item . attrs ()) ; combinations . insert (traits) ; } } } }
};
}
