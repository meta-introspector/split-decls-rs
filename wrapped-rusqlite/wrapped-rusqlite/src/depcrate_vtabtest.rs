// Generated macro for test (module)
macro_rules! Depcrate_vtabtest {
() => {
// Module: crate::vtab
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { # [test] fn test_dequote () { assert_eq ! ("" , super :: dequote ("")) ; assert_eq ! ("'" , super :: dequote ("'")) ; assert_eq ! ("\"" , super :: dequote ("\"")) ; assert_eq ! ("'\"" , super :: dequote ("'\"")) ; assert_eq ! ("" , super :: dequote ("''")) ; assert_eq ! ("" , super :: dequote ("\"\"")) ; assert_eq ! ("x" , super :: dequote ("'x'")) ; assert_eq ! ("x" , super :: dequote ("\"x\"")) ; assert_eq ! ("x" , super :: dequote ("x")) ; } # [test] fn test_parse_boolean () { assert_eq ! (None , super :: parse_boolean ("")) ; assert_eq ! (Some (true) , super :: parse_boolean ("1")) ; assert_eq ! (Some (true) , super :: parse_boolean ("yes")) ; assert_eq ! (Some (true) , super :: parse_boolean ("on")) ; assert_eq ! (Some (true) , super :: parse_boolean ("true")) ; assert_eq ! (Some (false) , super :: parse_boolean ("0")) ; assert_eq ! (Some (false) , super :: parse_boolean ("no")) ; assert_eq ! (Some (false) , super :: parse_boolean ("off")) ; assert_eq ! (Some (false) , super :: parse_boolean ("false")) ; } # [test] fn test_parse_parameters () { assert_eq ! (Ok (("key" , "value")) , super :: parameter (b"key='value'")) ; assert_eq ! (Ok (("key" , "foo=bar")) , super :: parameter (b"key='foo=bar'")) ; } }
};
}
