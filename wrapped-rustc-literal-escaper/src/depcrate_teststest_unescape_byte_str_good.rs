// Generated macro for test_unescape_byte_str_good (function)
macro_rules! Depcrate_teststest_unescape_byte_str_good {
() => {
// Module: crate::tests
// Provides: {"test_unescape_byte_str_good"}
// Dependencies: {}
# [test] fn test_unescape_byte_str_good () { fn check (literal_text : & str , expected : & [u8]) { let mut result = Ok (Vec :: with_capacity (literal_text . len ())) ; unescape_byte_str (literal_text , | range , res | { if let Ok (buf) = & mut result { match res { Ok (b) => buf . push (b) , Err (e) => result = Err ((range , e)) , } } }) ; assert_eq ! (result . as_deref () , Ok (expected)) } check ("foo" , b"foo") ; check ("" , b"") ; check (" \t\n" , b" \t\n") ; check ("hello \\\n     world" , b"hello world") ; check ("thread's" , b"thread's") }
};
}
