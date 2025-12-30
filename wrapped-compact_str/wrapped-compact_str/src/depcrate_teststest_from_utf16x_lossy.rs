// Generated macro for test_from_utf16x_lossy (function)
macro_rules! Depcrate_teststest_from_utf16x_lossy {
() => {
// Module: crate::tests
// Provides: {"test_from_utf16x_lossy"}
// Dependencies: {}
# [test] fn test_from_utf16x_lossy () { let dancing_men = b"\x3d\xd8\x6f\xfc\x0d\x20\x42\x26\x0f\xfe" ; assert_eq ! (CompactString :: from_utf16le_lossy (dancing_men) , "�\u{fc6f}\u{200d}♂️" ,) ; let dancing_men = b"0\x3d\xd8\x6f\xfc\x0d\x20\x42\x26\x0f\xfe" ; assert_eq ! (CompactString :: from_utf16le_lossy (& dancing_men [1 ..]) , "�\u{fc6f}\u{200d}♂️" ,) ; let dancing_women = b"\xd8\x3d\xdc\x6f\x20\x0d\x26\x40\xde\x0f" ; assert_eq ! (CompactString :: from_utf16be_lossy (dancing_women) , "👯\u{200d}♀�" ,) ; let dancing_women = b"0\xd8\x3d\xdc\x6f\x20\x0d\x26\x40\xde\x0f" ; assert_eq ! (CompactString :: from_utf16be_lossy (& dancing_women [1 ..]) , "👯\u{200d}♀�" ,) ; }
};
}
