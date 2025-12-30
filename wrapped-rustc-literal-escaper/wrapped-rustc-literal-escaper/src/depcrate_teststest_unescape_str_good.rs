// Generated macro for test_unescape_str_good (function)
macro_rules! Depcrate_teststest_unescape_str_good {
() => {
// Module: crate::tests
// Provides: {"test_unescape_str_good"}
// Dependencies: {}
# [test] fn test_unescape_str_good () { fn check (literal_text : & str , expected : & str) { let mut buf = Ok (String :: with_capacity (literal_text . len ())) ; unescape_str (literal_text , | range , c | { if let Ok (b) = & mut buf { match c { Ok (c) => b . push (c) , Err (e) => buf = Err ((range , e)) , } } }) ; assert_eq ! (buf . as_deref () , Ok (expected)) } check ("foo" , "foo") ; check ("" , "") ; check (" \t\n" , " \t\n") ; check ("hello \\\n     world" , "hello world") ; check ("thread's" , "thread's") }
};
}
