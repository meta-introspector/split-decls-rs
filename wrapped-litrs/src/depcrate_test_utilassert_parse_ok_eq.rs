// Generated macro for assert_parse_ok_eq (function)
macro_rules! Depcrate_test_utilassert_parse_ok_eq {
() => {
// Module: crate::test_util
// Provides: {"assert_parse_ok_eq"}
// Dependencies: {}
# [track_caller] pub (crate) fn assert_parse_ok_eq < T : PartialEq + Debug + Display > (input : & str , result : Result < T , ParseError > , expected : T , parse_method : & str ,) { match result { Ok (actual) if actual == expected => { if actual . to_string () != input { panic ! ("formatting does not yield original input `{input}`: {actual:?}") ; } } Ok (actual) => { panic ! ("unexpected parsing result (with `{parse_method}`) for `{input}`:\n\
                actual:    {actual:?}\n\
                expected:  {expected:?}") ; } Err (e) => { panic ! ("expected `{input}` to be parsed (with `{parse_method}`) successfully, \
                but it failed: {e:?}") ; } } }
};
}
