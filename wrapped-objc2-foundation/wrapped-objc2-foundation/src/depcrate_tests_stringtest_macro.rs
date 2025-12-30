// Generated macro for test_macro (function)
macro_rules! Depcrate_tests_stringtest_macro {
() => {
// Module: crate::tests::string
// Provides: {"test_macro"}
// Dependencies: {}
# [test] fn test_macro () { macro_rules ! test { ($ ($ s : expr ,) +) => { $ ({ let s1 = ns_string ! ($ s) ; let s2 = NSString :: from_str ($ s) ; assert_eq ! (s1 , s1) ; assert_eq ! (s1 , &* s2) ; assert_eq ! (s1 . to_string () , $ s) ; assert_eq ! (s2 . to_string () , $ s) ; }) + } ; } test ! { "" , "asdf" , "🦀" , "🏳️‍🌈" , "𝄞music" , "abcd【e】fg" , "abcd⒠fg" , "ääääh" , "lööps, bröther?" , "\u{fffd} \u{fffd} \u{fffd}" , "讓每個人都能打造出。" , "\0" , "\0\x01\x02\x03\x04\x05\x06\x07\x08\x09" , include_str ! ("string.rs") , } }
};
}
