// Generated macro for test_copy_mutable (function)
macro_rules! Depcrate_tests_attributed_stringtest_copy_mutable {
() => {
// Module: crate::tests::attributed_string
// Provides: {"test_copy_mutable"}
// Dependencies: {}
# [test] # [cfg_attr (feature = "gnustep-1-7" , ignore = "thread safety issues regarding initialization")] fn test_copy_mutable () { use crate :: { NSCopying , NSMutableCopying , NSObjectProtocol } ; let s1 = NSMutableAttributedString :: from_nsstring (ns_string ! ("abc")) ; let s2 = s1 . copy () ; assert_ne ! (Retained :: as_ptr (& s1) . cast () , Retained :: as_ptr (& s2)) ; assert ! (s2 . isKindOfClass (NSAttributedString :: class ())) ; let s3 = s1 . mutableCopy () ; assert_ne ! (Retained :: as_ptr (& s1) , Retained :: as_ptr (& s3)) ; assert ! (s3 . isKindOfClass (NSMutableAttributedString :: class ())) ; }
};
}
