// Generated macro for test_generic_auto_traits (function)
macro_rules! Depcrate_tests_auto_traitstest_generic_auto_traits {
() => {
// Module: crate::tests::auto_traits
// Provides: {"test_generic_auto_traits"}
// Dependencies: {}
# [test] fn test_generic_auto_traits () { # [cfg (feature = "NSArray")] assert_not_impl_any ! (crate :: NSArray < AnyObject >: Unpin) ; # [cfg (feature = "NSArray")] assert_not_impl_any ! (crate :: NSMutableArray < AnyObject >: Unpin) ; # [cfg (feature = "NSDictionary")] assert_not_impl_any ! (crate :: NSDictionary < AnyObject , AnyObject >: Unpin) ; # [cfg (feature = "NSArray")] assert_not_impl_any ! (crate :: NSArray < SendSyncObject >: Send , Sync) ; # [cfg (feature = "NSArray")] assert_not_impl_any ! (crate :: NSMutableArray < SendSyncObject >: Send , Sync) ; # [cfg (feature = "NSDictionary")] assert_not_impl_any ! (crate :: NSDictionary < SendSyncObject , SendSyncObject >: Send , Sync) ; # [cfg (feature = "NSProcessInfo")] { use crate :: NSProcessInfo ; # [cfg (feature = "NSDictionary")] assert_not_impl_any ! (crate :: NSDictionary < NSProcessInfo , NSProcessInfo >: UnwindSafe , RefUnwindSafe) ; # [cfg (feature = "NSSet")] assert_not_impl_any ! (crate :: NSSet < NSProcessInfo >: UnwindSafe , RefUnwindSafe) ; # [cfg (feature = "NSSet")] assert_not_impl_any ! (Retained < crate :: NSSet < NSProcessInfo >>: UnwindSafe , RefUnwindSafe) ; # [cfg (feature = "NSArray")] assert_not_impl_any ! (crate :: NSMutableArray < NSProcessInfo >: UnwindSafe , RefUnwindSafe) ; # [cfg (feature = "NSDictionary")] assert_not_impl_any ! (crate :: NSMutableDictionary < NSProcessInfo , NSProcessInfo >: UnwindSafe , RefUnwindSafe) ; # [cfg (feature = "NSSet")] assert_not_impl_any ! (crate :: NSMutableSet < NSProcessInfo >: UnwindSafe , RefUnwindSafe) ; } }
};
}
