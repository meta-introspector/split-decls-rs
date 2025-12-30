// Generated macro for tests (module)
macro_rules! Depcrate_util_parse_attributetests {
() => {
// Module: crate::util::parse_attribute
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: parse_attribute_to_meta_list ; use crate :: ast :: NestedMeta ; use syn :: spanned :: Spanned ; use syn :: { parse_quote , Ident } ; # [test] fn parse_list () { let meta = parse_attribute_to_meta_list (& parse_quote ! (# [bar (baz = 4)])) . unwrap () ; let nested_meta = NestedMeta :: parse_meta_list (meta . tokens) . unwrap () ; assert_eq ! (nested_meta . len () , 1) ; } # [test] fn parse_path_returns_empty_list () { let meta = parse_attribute_to_meta_list (& parse_quote ! (# [bar])) . unwrap () ; let nested_meta = NestedMeta :: parse_meta_list (meta . tokens) . unwrap () ; assert ! (meta . path . is_ident (& Ident :: new ("bar" , meta . path . span ()))) ; assert ! (nested_meta . is_empty ()) ; } # [test] fn parse_name_value_returns_error () { parse_attribute_to_meta_list (& parse_quote ! (# [bar = 4])) . unwrap_err () ; } # [test] fn parse_name_value_error_includes_example () { let err = parse_attribute_to_meta_list (& parse_quote ! (# [bar = 4])) . unwrap_err () ; assert ! (err . to_string () . contains ("#[bar(...)]")) ; } }
};
}
