// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_decode () { fn test_one (input : & [u8] , expected_result : & str , expected_encoding : & str) { let (result , used_encoding) = decode (input , DecoderTrap :: Strict , all :: ISO_8859_1 as EncodingRef) ; let result = result . unwrap () ; assert_eq ! (used_encoding . name () , expected_encoding) ; assert_eq ! (& result [..] , expected_result) ; } test_one (& [0xEF , 0xBB , 0xBF , 0xC3 , 0xA9] , "é" , "utf-8") ; test_one (& [0xC3 , 0xA9] , "Ã©" , "iso-8859-1") ; test_one (& [0xFE , 0xFF , 0x00 , 0xE9] , "é" , "utf-16be") ; test_one (& [0x00 , 0xE9] , "\x00é" , "iso-8859-1") ; test_one (& [0xFF , 0xFE , 0xE9 , 0x00] , "é" , "utf-16le") ; test_one (& [0xE9 , 0x00] , "é\x00" , "iso-8859-1") ; } }
};
}
