// Generated macro for test_basic (function)
macro_rules! Depcrate_uint_iteratortest_basic {
() => {
// Module: crate::uint_iterator
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { let mut it = IntIterator { unum : 123usize , is_negative : false , } ; assert_eq ! (Some (3) , it . next ()) ; assert_eq ! (Some (2) , it . next ()) ; assert_eq ! (Some (1) , it . next ()) ; assert_eq ! (None , it . next ()) ; }
};
}
