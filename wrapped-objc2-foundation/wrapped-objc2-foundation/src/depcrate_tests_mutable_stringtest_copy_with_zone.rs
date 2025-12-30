// Generated macro for test_copy_with_zone (function)
macro_rules! Depcrate_tests_mutable_stringtest_copy_with_zone {
() => {
// Module: crate::tests::mutable_string
// Provides: {"test_copy_with_zone"}
// Dependencies: {}
# [test] # [cfg (all (feature = "NSObject" , feature = "NSZone"))] fn test_copy_with_zone () { use crate :: { NSCopying , NSMutableCopying , NSObjectProtocol } ; use objc2 :: { rc :: Retained , ClassType } ; let s1 = NSString :: from_str ("abc") ; let s2 = unsafe { s1 . copyWithZone (core :: ptr :: null_mut ()) } ; assert_eq ! (Retained :: as_ptr (& s1) , Retained :: as_ptr (& s2)) ; assert ! (s2 . isKindOfClass (NSString :: class ())) ; let s3 = unsafe { s1 . mutableCopyWithZone (core :: ptr :: null_mut ()) } ; assert_ne ! (Retained :: as_ptr (& s1) . cast ::< NSMutableString > () , Retained :: as_ptr (& s3)) ; assert ! (s3 . isKindOfClass (NSMutableString :: class ())) ; }
};
}
