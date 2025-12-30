// Generated macro for parse_from_deprecated_attr_test (module)
macro_rules! Depcrate_common_deprecationparse_from_deprecated_attr_test {
() => {
// Module: crate::common::deprecation
// Provides: {"parse_from_deprecated_attr_test"}
// Dependencies: {}
# [cfg (test)] mod parse_from_deprecated_attr_test { use quote :: quote ; use syn :: parse_quote ; use super :: Directive ; # [test] fn single () { let desc = Directive :: parse_from_deprecated_attr (& [parse_quote ! { # [deprecated (note = "foo")] }]) . unwrap () . unwrap () . into_inner () ; assert_eq ! (quote ! { # desc } . to_string () , quote ! { . deprecated (:: core :: option :: Option :: Some (:: juniper :: arcstr :: literal ! ("foo"))) } . to_string () ,) ; } # [test] fn no_reason () { let desc = Directive :: parse_from_deprecated_attr (& [parse_quote ! { # [deprecated] }]) . unwrap () . unwrap () . into_inner () ; assert_eq ! (quote ! { # desc } . to_string () , quote ! { . deprecated (:: core :: option :: Option ::<:: juniper :: ArcStr >:: None) } . to_string () ,) ; } # [test] fn not_deprecation () { let desc = Directive :: parse_from_deprecated_attr (& [parse_quote ! { # [blah = "foo"] }]) . unwrap () ; assert_eq ! (desc , None) ; } }
};
}
