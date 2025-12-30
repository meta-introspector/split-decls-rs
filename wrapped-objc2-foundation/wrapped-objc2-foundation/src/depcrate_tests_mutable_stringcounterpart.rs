// Generated macro for counterpart (function)
macro_rules! Depcrate_tests_mutable_stringcounterpart {
() => {
// Module: crate::tests::mutable_string
// Provides: {"counterpart"}
// Dependencies: {}
# [test] # [cfg (feature = "NSObject")] fn counterpart () { use crate :: { CopyingHelper , MutableCopyingHelper } ; use core :: any :: TypeId ; assert_eq ! (TypeId :: of ::<< NSString as CopyingHelper >:: Result > () , TypeId :: of ::< NSString > () ,) ; assert_eq ! (TypeId :: of ::<< NSString as MutableCopyingHelper >:: Result > () , TypeId :: of ::< NSMutableString > () ,) ; assert_eq ! (TypeId :: of ::<< NSMutableString as CopyingHelper >:: Result > () , TypeId :: of ::< NSString > () ,) ; assert_eq ! (TypeId :: of ::<< NSMutableString as MutableCopyingHelper >:: Result > () , TypeId :: of ::< NSMutableString > () ,) ; }
};
}
