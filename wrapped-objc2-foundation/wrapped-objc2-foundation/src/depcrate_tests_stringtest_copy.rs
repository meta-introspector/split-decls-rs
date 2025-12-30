// Generated macro for test_copy (function)
macro_rules! Depcrate_tests_stringtest_copy {
() => {
// Module: crate::tests::string
// Provides: {"test_copy"}
// Dependencies: {}
# [test] # [cfg (feature = "NSObject")] fn test_copy () { use crate :: { NSCopying , NSMutableCopying , NSMutableString , NSObjectProtocol } ; use objc2 :: { rc :: Retained , ClassType } ; let s1 = NSString :: from_str ("abc") ; let s2 = s1 . copy () ; assert_eq ! (Retained :: as_ptr (& s1) , Retained :: as_ptr (& s2)) ; assert ! (s2 . isKindOfClass (NSString :: class ())) ; let s3 = s1 . mutableCopy () ; assert_ne ! (Retained :: as_ptr (& s1) , Retained :: as_ptr (& s3) . cast ()) ; assert ! (s3 . isKindOfClass (NSMutableString :: class ())) ; }
};
}
