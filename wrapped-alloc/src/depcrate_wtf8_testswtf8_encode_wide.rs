// Generated macro for wtf8_encode_wide (function)
macro_rules! Depcrate_wtf8_testswtf8_encode_wide {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_encode_wide"}
// Dependencies: {}
# [test] fn wtf8_encode_wide () { let mut string = Wtf8Buf :: from_str ("aé ") ; string . push (CodePoint :: from_u32 (0xD83D) . unwrap ()) ; string . push_char ('💩') ; assert_eq ! (string . encode_wide () . collect ::< Vec < _ >> () , vec ! [0x61 , 0xE9 , 0x20 , 0xD83D , 0xD83D , 0xDCA9]) ; }
};
}
