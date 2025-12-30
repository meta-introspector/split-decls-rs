// Generated macro for wtf8buf_extend (function)
macro_rules! Depcrate_wtf8_testswtf8buf_extend {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8buf_extend"}
// Dependencies: {}
# [test] fn wtf8buf_extend () { fn e (initial : & [u32] , extended : & [u32]) -> Wtf8Buf { fn c (value : & u32) -> CodePoint { CodePoint :: from_u32 (* value) . unwrap () } let mut string = initial . iter () . map (c) . collect :: < Wtf8Buf > () ; string . extend (extended . iter () . map (c)) ; string } assert_eq ! (e (& [0x61 , 0xE9] , & [0x20 , 0x1F4A9]) , Wtf8Buf { bytes : b"a\xC3\xA9 \xF0\x9F\x92\xA9" . to_vec () , is_known_utf8 : true }) ; assert_eq ! (e (& [0xD83D] , & [0xDCA9]) . as_bytes () , b"\xF0\x9F\x92\xA9") ; assert_eq ! (e (& [0xD83D , 0x20] , & [0xDCA9]) , Wtf8Buf { bytes : b"\xED\xA0\xBD \xED\xB2\xA9" . to_vec () , is_known_utf8 : false }) ; assert_eq ! (e (& [0xD800] , & [0xDBFF]) , Wtf8Buf { bytes : b"\xED\xA0\x80\xED\xAF\xBF" . to_vec () , is_known_utf8 : false }) ; assert_eq ! (e (& [0xD800] , & [0xE000]) , Wtf8Buf { bytes : b"\xED\xA0\x80\xEE\x80\x80" . to_vec () , is_known_utf8 : false }) ; assert_eq ! (e (& [0xD7FF] , & [0xDC00]) , Wtf8Buf { bytes : b"\xED\x9F\xBF\xED\xB0\x80" . to_vec () , is_known_utf8 : false }) ; assert_eq ! (e (& [0x61] , & [0xDC00]) , Wtf8Buf { bytes : b"\x61\xED\xB0\x80" . to_vec () , is_known_utf8 : false }) ; assert_eq ! (e (& [] , & [0xDC00]) , Wtf8Buf { bytes : b"\xED\xB0\x80" . to_vec () , is_known_utf8 : false }) ; }
};
}
