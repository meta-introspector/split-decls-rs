// Generated macro for write_same_string (function)
macro_rules! Depcrate_tests_mutable_stringwrite_same_string {
() => {
// Module: crate::tests::mutable_string
// Provides: {"write_same_string"}
// Dependencies: {}
# [doc = " Test that writing a NSMutableString to itself works (i.e. ensure that"] # [doc = " `NSString`'s `Debug` implementation is correct)."] # [test] fn write_same_string () { let string = NSMutableString :: from_str ("foo") ; let mut expected = String :: from ("foo") ; for i in 0 .. 3 { write ! (& string , "{string}") . unwrap () ; write ! (& string , "{string:?}") . unwrap () ; let object : & NSObject = & string ; write ! (& string , "{object:?}") . unwrap () ; let expected_clone = expected . clone () ; write ! (& mut expected , "{expected_clone}") . unwrap () ; let expected_clone = expected . clone () ; write ! (& mut expected , "{expected_clone:?}") . unwrap () ; let expected_clone = expected . clone () ; write ! (& mut expected , "{expected_clone}") . unwrap () ; assert_eq ! (string . to_string () , expected , "failed at iteration {i}") ; } }
};
}
