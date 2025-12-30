// Generated macro for test_copy (function)
macro_rules! Depcrate_tests_mutable_stringtest_copy {
() => {
// Module: crate::tests::mutable_string
// Provides: {"test_copy"}
// Dependencies: {}
# [test] # [cfg (feature = "NSObject")] fn test_copy () { use crate :: { NSCopying , NSMutableCopying , NSObjectProtocol } ; use objc2 :: { rc :: Retained , ClassType } ; let s1 = NSMutableString :: from_str ("abc") ; let s2 = s1 . copy () ; assert_ne ! (Retained :: as_ptr (& s1) , Retained :: as_ptr (& s2) . cast ()) ; assert ! (s2 . isKindOfClass (NSString :: class ())) ; let s3 = s1 . mutableCopy () ; assert_ne ! (Retained :: as_ptr (& s1) , Retained :: as_ptr (& s3)) ; assert ! (s3 . isKindOfClass (NSMutableString :: class ())) ; }
};
}
