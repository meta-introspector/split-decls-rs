// Generated macro for should (module)
macro_rules! Depcrate_parse_vlistshould {
() => {
// Module: crate::parse::vlist
// Provides: {"should"}
// Dependencies: {}
# [cfg (test)] mod should { use crate :: test :: { assert_eq , * } ; use super :: * ; mod parse_values_list { use super :: assert_eq ; use super :: * ; fn parse_values_list < S : AsRef < str > > (values_list : S) -> ValueList { parse_meta (values_list) } # [test] fn some_literals () { let literals = literal_expressions_str () ; let name = "argument" ; let values_list = parse_values_list (format ! (r#"{} => [{}]"# , name , literals . iter () . map (ToString :: to_string) . collect ::< Vec < String >> () . join (", "))) ; assert_eq ! (name , & values_list . arg . display_code ()) ; assert_eq ! (values_list . args () , to_args ! (literals)) ; } # [test] fn raw_code () { let values_list = parse_values_list (r#"no_mater => [vec![1,2,3]]"#) ; assert_eq ! (values_list . args () , to_args ! (["vec![1, 2, 3]"])) ; } # [test] # [should_panic] fn raw_code_with_parsing_error () { parse_values_list (r#"other => [some:<>(1,2,3)]"#) ; } # [test] # [should_panic (expected = r#"expected square brackets"#)] fn forget_brackets () { parse_values_list (r#"other => 42"#) ; } } }
};
}
