// Generated macro for test_is_empty (function)
macro_rules! Depcrate_teststest_is_empty {
() => {
// Module: crate::tests
// Provides: {"test_is_empty"}
// Dependencies: {}
# [test] fn test_is_empty () { const ZEROS : & [& str] = & ["\0" , "\0\0\0\0\0\0\0\0\0\0\0\0" , "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0" , "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0" ,] ; assert ! (CompactString :: new ("") . is_empty ()) ; assert ! (CompactString :: const_new ("") . is_empty ()) ; for (len , s) in ZEROS . iter () . copied () . enumerate () { let mut a = CompactString :: new (s) ; let mut b = CompactString :: new (s) ; for _ in (1 ..= len) . rev () { a . truncate (len) ; b . truncate (len) ; assert ! (! a . is_empty ()) ; assert ! (! b . is_empty ()) ; } } }
};
}
