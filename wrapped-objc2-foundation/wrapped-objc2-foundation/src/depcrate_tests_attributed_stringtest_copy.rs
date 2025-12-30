// Generated macro for test_copy (function)
macro_rules! Depcrate_tests_attributed_stringtest_copy {
() => {
// Module: crate::tests::attributed_string
// Provides: {"test_copy"}
// Dependencies: {}
# [test] fn test_copy () { use crate :: { NSCopying , NSMutableCopying , NSObjectProtocol } ; let s1 = NSAttributedString :: from_nsstring (ns_string ! ("abc")) ; let s2 = s1 . copy () ; assert ! (s2 . isKindOfClass (NSAttributedString :: class ())) ; let s3 = s1 . mutableCopy () ; assert_ne ! (Retained :: as_ptr (& s1) , Retained :: as_ptr (& s3) . cast ()) ; assert ! (s3 . isKindOfClass (NSMutableAttributedString :: class ())) ; }
};
}
