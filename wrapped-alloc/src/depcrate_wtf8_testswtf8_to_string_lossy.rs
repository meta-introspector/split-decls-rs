// Generated macro for wtf8_to_string_lossy (function)
macro_rules! Depcrate_wtf8_testswtf8_to_string_lossy {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_to_string_lossy"}
// Dependencies: {}
# [test] fn wtf8_to_string_lossy () { assert_eq ! (to_string_lossy (Wtf8 :: from_str ("")) , Cow :: Borrowed ("")) ; assert_eq ! (to_string_lossy (Wtf8 :: from_str ("aé 💩")) , Cow :: Borrowed ("aé 💩")) ; let mut string = Wtf8Buf :: from_str ("aé 💩") ; string . push (CodePoint :: from_u32 (0xD800) . unwrap ()) ; let expected : Cow < '_ , str > = Cow :: Owned (String :: from ("aé 💩�")) ; assert_eq ! (to_string_lossy (& string) , expected) ; }
};
}
