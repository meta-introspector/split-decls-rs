// Generated macro for test (module)
macro_rules! Depcrate_path_parsertest {
() => {
// Module: crate::path::parser
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use snapbox :: prelude :: * ; use snapbox :: { assert_data_eq , str } ; use super :: * ; # [test] fn test_id () { let parsed : Expression = from_str ("abcd") . unwrap () ; assert_data_eq ! (parsed . to_debug () , str ! [[r#"
Expression {
    root: "abcd",
    postfix: [],
}

"#]]) ; } # [test] fn test_id_dash () { let parsed : Expression = from_str ("abcd-efgh") . unwrap () ; assert_data_eq ! (parsed . to_debug () , str ! [[r#"
Expression {
    root: "abcd-efgh",
    postfix: [],
}

"#]]) ; } # [test] fn test_child () { let parsed : Expression = from_str ("abcd.efgh") . unwrap () ; assert_data_eq ! (parsed . to_debug () , str ! [[r#"
Expression {
    root: "abcd",
    postfix: [
        Key(
            "efgh",
        ),
    ],
}

"#]]) ; let parsed : Expression = from_str ("abcd.efgh.ijkl") . unwrap () ; assert_data_eq ! (parsed . to_debug () , str ! [[r#"
Expression {
    root: "abcd",
    postfix: [
        Key(
            "efgh",
        ),
        Key(
            "ijkl",
        ),
    ],
}

"#]]) ; } # [test] fn test_subscript () { let parsed : Expression = from_str ("abcd[12]") . unwrap () ; assert_data_eq ! (parsed . to_debug () , str ! [[r#"
Expression {
    root: "abcd",
    postfix: [
        Index(
            12,
        ),
    ],
}

"#]]) ; } # [test] fn test_subscript_neg () { let parsed : Expression = from_str ("abcd[-1]") . unwrap () ; assert_data_eq ! (parsed . to_debug () , str ! [[r#"
Expression {
    root: "abcd",
    postfix: [
        Index(
            -1,
        ),
    ],
}

"#]]) ; } # [test] fn test_invalid_identifier () { let err = from_str ("!") . unwrap_err () ; assert_data_eq ! (err . to_string () , str ! [[r#"
!
^
invalid identifier
expected ASCII alphanumeric, `_`, `-`
"#]]) ; } # [test] fn test_invalid_child () { let err = from_str ("a..") . unwrap_err () ; assert_data_eq ! (err . to_string () , str ! [[r#"
a..
  ^
invalid identifier
expected ASCII alphanumeric, `_`, `-`
"#]]) ; } # [test] fn test_invalid_subscript () { let err = from_str ("a[b]") . unwrap_err () ; assert_data_eq ! (err . to_string () , str ! [[r#"
a[b]
  ^
invalid subscript
expected integer
"#]]) ; } # [test] fn test_incomplete_subscript () { let err = from_str ("a[0") . unwrap_err () ; assert_data_eq ! (err . to_string () , str ! [[r#"
a[0
   ^
invalid subscript
expected `]`
"#]]) ; } # [test] fn test_invalid_postfix () { let err = from_str ("a!b") . unwrap_err () ; assert_data_eq ! (err . to_string () , str ! [[r#"
a!b
  ^
invalid postfix
expected `[`, `.`
"#]]) ; } }
};
}
