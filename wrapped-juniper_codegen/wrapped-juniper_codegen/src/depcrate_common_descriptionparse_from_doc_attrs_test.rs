// Generated macro for parse_from_doc_attrs_test (module)
macro_rules! Depcrate_common_descriptionparse_from_doc_attrs_test {
() => {
// Module: crate::common::description
// Provides: {"parse_from_doc_attrs_test"}
// Dependencies: {}
# [cfg (test)] mod parse_from_doc_attrs_test { use quote :: quote ; use syn :: parse_quote ; use super :: Description ; # [test] fn single () { let desc = Description :: parse_from_doc_attrs (& [parse_quote ! { # [doc = "foo"] }]) . unwrap () . unwrap () . into_inner () ; assert_eq ! (quote ! { # desc } . to_string () , quote ! { . description (:: juniper :: arcstr :: literal ! ("foo")) } . to_string () ,) ; } # [test] fn many () { let desc = Description :: parse_from_doc_attrs (& [parse_quote ! { # [doc = "foo"] } , parse_quote ! { # [doc = "\n"] } , parse_quote ! { # [doc = "bar"] } ,]) . unwrap () . unwrap () . into_inner () ; assert_eq ! (quote ! { # desc } . to_string () , quote ! { . description (:: juniper :: arcstr :: literal ! ("foo\n\nbar")) } . to_string () ,) ; } # [test] fn not_doc () { let desc = Description :: parse_from_doc_attrs (& [parse_quote ! { # [blah = "foo"] }]) . unwrap () ; assert_eq ! (desc , None) ; } }
};
}
