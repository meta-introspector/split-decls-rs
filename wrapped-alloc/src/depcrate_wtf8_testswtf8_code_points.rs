// Generated macro for wtf8_code_points (function)
macro_rules! Depcrate_wtf8_testswtf8_code_points {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_code_points"}
// Dependencies: {}
# [test] fn wtf8_code_points () { fn c (value : u32) -> CodePoint { CodePoint :: from_u32 (value) . unwrap () } fn cp (string : & Wtf8Buf) -> Vec < Option < char > > { string . code_points () . map (| c | c . to_char ()) . collect :: < Vec < _ > > () } let mut string = Wtf8Buf :: from_str ("é ") ; assert_eq ! (cp (& string) , [Some ('é') , Some (' ')]) ; string . push (c (0xD83D)) ; assert_eq ! (cp (& string) , [Some ('é') , Some (' ') , None]) ; string . push (c (0xDCA9)) ; assert_eq ! (cp (& string) , [Some ('é') , Some (' ') , Some ('💩')]) ; }
};
}
