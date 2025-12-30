// Generated macro for test_parse_trailer (module)
macro_rules! Depcrate_commit_message_bodytest_parse_trailer {
() => {
// Module: crate::commit::message::body
// Provides: {"test_parse_trailer"}
// Dependencies: {}
# [cfg (test)] mod test_parse_trailer { use super :: * ; fn parse (input : & str) -> (& BStr , & BStr) { parse_single_line_trailer :: < () > . parse_peek (input . as_bytes ()) . unwrap () . 1 } # [test] fn simple_newline () { assert_eq ! (parse ("foo: bar\n") , ("foo" . into () , "bar" . into ())) ; } # [test] fn simple_non_ascii_no_newline () { assert_eq ! (parse ("🤗: 🎉") , ("🤗" . into () , "🎉" . into ())) ; } # [test] fn with_lots_of_whitespace_newline () { assert_eq ! (parse ("hello foo: bar there   \n") , ("hello foo" . into () , "bar there" . into ())) ; } # [test] fn extra_whitespace_before_token_or_value_is_error () { assert ! (parse_single_line_trailer ::< () >. parse_peek (b"foo : bar") . is_err ()) ; assert ! (parse_single_line_trailer ::< () >. parse_peek (b"foo:  bar") . is_err ()) ; } # [test] fn simple_newline_windows () { assert_eq ! (parse ("foo: bar\r\n") , ("foo" . into () , "bar" . into ())) ; } }
};
}
