// Generated macro for test_from_utf16x (function)
macro_rules! Depcrate_teststest_from_utf16x {
() => {
// Module: crate::tests
// Provides: {"test_from_utf16x"}
// Dependencies: {}
# [test] fn test_from_utf16x () { let dancing_men = b"\x3d\xd8\x6f\xdc\x0d\x20\x42\x26\x0f\xfe" ; assert_eq ! (CompactString :: from_utf16le (dancing_men) . unwrap () , "👯‍♂️") ; let dancing_men = b"0\x3d\xd8\x6f\xdc\x0d\x20\x42\x26\x0f\xfe" ; assert ! (CompactString :: from_utf16le (dancing_men) . is_err ()) ; assert_eq ! (CompactString :: from_utf16le (& dancing_men [1 ..]) . unwrap () , "👯‍♂️" ,) ; let dancing_women = b"\xd8\x3d\xdc\x6f\x20\x0d\x26\x40\xfe\x0f" ; assert_eq ! (CompactString :: from_utf16be (dancing_women) . unwrap () , "👯‍♀️") ; let dancing_women = b"0\xd8\x3d\xdc\x6f\x20\x0d\x26\x40\xfe\x0f" ; assert ! (CompactString :: from_utf16be (dancing_women) . is_err ()) ; assert_eq ! (CompactString :: from_utf16be (& dancing_women [1 ..]) . unwrap () , "👯‍♀️" ,) ; }
};
}
