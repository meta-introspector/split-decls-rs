// Generated macro for test_copy_nsstring_is_same (function)
macro_rules! Depcrate_tests_stringtest_copy_nsstring_is_same {
() => {
// Module: crate::tests::string
// Provides: {"test_copy_nsstring_is_same"}
// Dependencies: {}
# [test] # [cfg (feature = "NSObject")] fn test_copy_nsstring_is_same () { use crate :: NSCopying ; let string1 = NSString :: from_str ("Hello, world!") ; let string2 = string1 . copy () ; assert ! (core :: ptr :: eq (&* string1 , &* string2) , "Cloned NSString didn't have the same address") ; }
};
}
