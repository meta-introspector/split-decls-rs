// Generated macro for test_allowed_mutation_while_iterating (function)
macro_rules! Depcrate_tests_mutable_arraytest_allowed_mutation_while_iterating {
() => {
// Module: crate::tests::mutable_array
// Provides: {"test_allowed_mutation_while_iterating"}
// Dependencies: {}
# [test] # [cfg (feature = "NSString")] # [cfg_attr (feature = "gnustep-1-7" , ignore = "thread safety issues regarding initialization")] fn test_allowed_mutation_while_iterating () { use crate :: { NSMutableString , NSString } ; let array = NSMutableArray :: from_retained_slice (& [NSMutableString :: new () , NSMutableString :: new ()]) ; let to_add = NSString :: from_str ("test") ; for s in & array { s . appendString (& to_add) ; } assert_eq ! (array . objectAtIndex (0) , to_add) ; assert_eq ! (array . objectAtIndex (1) , to_add) ; }
};
}
