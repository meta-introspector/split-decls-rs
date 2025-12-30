// Generated macro for null_itemsptr (function)
macro_rules! Depcrate_tests_mutable_arraynull_itemsptr {
() => {
// Module: crate::tests::mutable_array
// Provides: {"null_itemsptr"}
// Dependencies: {}
# [test] # [cfg (feature = "NSValue")] # [cfg_attr (all (target_os = "macos" , target_arch = "x86") , should_panic = "mutation detected during enumeration")] # [cfg_attr (not (all (target_os = "macos" , target_arch = "x86")) , should_panic = "`itemsPtr` was NULL, likely due to mutation during iteration")] # [cfg_attr (feature = "gnustep-1-7" , ignore = "errors differently on GNUStep")] fn null_itemsptr () { use crate :: NSNumber ; let array = NSMutableArray :: new () ; let mut iter = array . iter () ; array . addObject (& * NSNumber :: new_i32 (0)) ; array . addObject (& * NSNumber :: new_i32 (0)) ; array . removeObjectAtIndex (0) ; array . addObject (& * NSNumber :: new_i32 (0)) ; let _ = iter . next () ; array . removeAllObjects () ; let _ = iter . next () ; }
};
}
