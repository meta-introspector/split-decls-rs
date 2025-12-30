// Generated macro for collect_trait_combinations (function)
macro_rules! Depcrate_arbitrary_in_macro_rulescollect_trait_combinations {
() => {
// Module: crate::arbitrary_in_macro_rules
// Provides: {"collect_trait_combinations"}
// Dependencies: {}
fn collect_trait_combinations (file : & File , combinations : & mut std :: collections :: BTreeSet < Vec < String > > ,) { for item in & file . items { if let Item :: Fn (fn_item) = item { if fn_item . attrs . iter () . any (| attr | attr . path () . is_ident ("test")) { collect_from_function (fn_item , combinations) ; } } } }
};
}
