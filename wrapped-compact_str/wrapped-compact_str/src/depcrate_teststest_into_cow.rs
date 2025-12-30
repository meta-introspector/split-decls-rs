// Generated macro for test_into_cow (function)
macro_rules! Depcrate_teststest_into_cow {
() => {
// Module: crate::tests
// Provides: {"test_into_cow"}
// Dependencies: {}
# [test] fn test_into_cow () { let og = "aaa" ; let compact = CompactString :: new (og) ; let cow : alloc :: borrow :: Cow < '_ , str > = compact . into () ; assert_eq ! (og , cow) ; }
};
}
