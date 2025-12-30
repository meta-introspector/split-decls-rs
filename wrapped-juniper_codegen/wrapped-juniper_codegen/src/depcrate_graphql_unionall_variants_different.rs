// Generated macro for all_variants_different (function)
macro_rules! Depcrate_graphql_unionall_variants_different {
() => {
// Module: crate::graphql_union
// Provides: {"all_variants_different"}
// Dependencies: {}
# [doc = " Checks whether all [GraphQL union][1] `variants` represent a different Rust"] # [doc = " type."] # [doc = ""] # [doc = " # Notice"] # [doc = ""] # [doc = " This is not an optimal implementation, as it's possible to bypass this check"] # [doc = " by using a full qualified path instead (`crate::Test` vs `Test`). Since this"] # [doc = " requirement is mandatory, the static assertion [`assert_type_ne_all!`][2] is"] # [doc = " used to enforce this requirement in the generated code. However, due to the"] # [doc = " bad error message this implementation should stay and provide guidance."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Unions"] # [doc = " [2]: juniper::sa::assert_type_ne_all"] fn all_variants_different (variants : & [VariantDefinition]) -> bool { let mut types : Vec < _ > = variants . iter () . map (| var | & var . ty) . collect () ; types . dedup () ; types . len () == variants . len () }
};
}
