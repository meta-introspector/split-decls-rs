// Generated macro for wtf8_display (function)
macro_rules! Depcrate_wtf8_testswtf8_display {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_display"}
// Dependencies: {}
# [test] fn wtf8_display () { fn d (b : & [u8]) -> String { (& unsafe { Wtf8 :: from_bytes_unchecked (b) }) . to_string () } assert_eq ! ("" , d ("" . as_bytes ())) ; assert_eq ! ("aé 💩" , d ("aé 💩" . as_bytes ())) ; let mut string = Wtf8Buf :: from_str ("aé 💩") ; string . push (CodePoint :: from_u32 (0xD800) . unwrap ()) ; assert_eq ! ("aé 💩�" , d (string . as_ref ())) ; }
};
}
