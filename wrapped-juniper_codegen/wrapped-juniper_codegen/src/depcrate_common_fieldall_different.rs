// Generated macro for all_different (function)
macro_rules! Depcrate_common_fieldall_different {
() => {
// Module: crate::common::field
// Provides: {"all_different"}
// Dependencies: {}
# [doc = " Checks whether all [GraphQL fields][1] fields have different names."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Language.Fields"] # [must_use] pub (crate) fn all_different (fields : & [Definition]) -> bool { let mut names : Vec < _ > = fields . iter () . map (| f | & f . name) . collect () ; names . dedup () ; names . len () == fields . len () }
};
}
