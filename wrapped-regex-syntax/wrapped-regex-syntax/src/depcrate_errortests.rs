// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use alloc :: string :: ToString ; use crate :: ast :: parse :: Parser ; fn assert_panic_message (pattern : & str , expected_msg : & str) { let result = Parser :: new () . parse (pattern) ; match result { Ok (_) => { panic ! ("regex should not have parsed") ; } Err (err) => { assert_eq ! (err . to_string () , expected_msg . trim ()) ; } } } # [test] fn regression_464 () { let err = Parser :: new () . parse ("a{\n") . unwrap_err () ; assert ! (! err . to_string () . is_empty ()) ; } # [test] fn repetition_quantifier_expects_a_valid_decimal () { assert_panic_message (r"\\u{[^}]*}" , r#"
regex parse error:
    \\u{[^}]*}
        ^
error: repetition quantifier expects a valid decimal
"# ,) ; } }
};
}
