// Generated macro for transform (function)
macro_rules! Depcrate_arbitrary_in_macro_rulestransform {
() => {
// Module: crate::arbitrary_in_macro_rules
// Provides: {"transform"}
// Dependencies: {}
fn transform (file : & mut File) -> Result < () > { let mut trait_combinations = std :: collections :: BTreeSet :: new () ; collect_trait_combinations (file , & mut trait_combinations) ; add_helper_macros (file , & trait_combinations) ; for item in & mut file . items { if let Item :: Fn (fn_item) = item { if fn_item . attrs . iter () . any (| attr | attr . path () . is_ident ("test")) { transform_test_function (fn_item) ; } } } Ok (()) }
};
}
