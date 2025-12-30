// Generated macro for test_from_cesu8 (function)
macro_rules! Depcratetest_from_cesu8 {
() => {
// Module: crate
// Provides: {"test_from_cesu8"}
// Dependencies: {}
# [test] fn test_from_cesu8 () { let data = & [0x4D , 0xE6 , 0x97 , 0xA5 , 0xED , 0xA0 , 0x81 , 0xED , 0xB0 , 0x81 , 0x7F] ; assert_eq ! (Cow :: Borrowed ("M日\u{10401}\u{7F}") , from_cesu8 (data) . unwrap ()) ; }
};
}
