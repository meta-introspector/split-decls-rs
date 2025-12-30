// Generated macro for tests (module)
macro_rules! Depcrate_token_writetests {
() => {
// Module: crate::token_write
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use alloc :: string :: String ; # [test] fn escape_debug () { let cases = [("hello" , r#"hello"#) , ("\\" , r#"\\"#) , ("\r" , r#"\r"#) , ("\n" , r#"\n"#) , ("\t" , r#"\t"#) , ("\"" , r#"\""#) , ("'" , r#"\'"#) , ("⛰️" , r#"⛰\u{fe0f}"#) ,] ; for (ai , ae) in cases { for (bi , be) in cases { let mut expected = String :: new () ; expected . push_str (ae) ; expected . push_str (be) ; let mut actual = String :: new () ; write_escape_debug (ai , | i | Ok (actual . push_str (i))) . unwrap () ; write_escape_debug (bi , | i | Ok (actual . push_str (i))) . unwrap () ; assert_eq ! (expected , actual) ; } } } }
};
}
