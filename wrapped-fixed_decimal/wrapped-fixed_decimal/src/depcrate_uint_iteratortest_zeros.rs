// Generated macro for test_zeros (function)
macro_rules! Depcrate_uint_iteratortest_zeros {
() => {
// Module: crate::uint_iterator
// Provides: {"test_zeros"}
// Dependencies: {}
# [test] fn test_zeros () { let mut it = IntIterator { unum : 9080usize , is_negative : false , } ; assert_eq ! (Some (0) , it . next ()) ; assert_eq ! (Some (8) , it . next ()) ; assert_eq ! (Some (0) , it . next ()) ; assert_eq ! (Some (9) , it . next ()) ; assert_eq ! (None , it . next ()) ; }
};
}
