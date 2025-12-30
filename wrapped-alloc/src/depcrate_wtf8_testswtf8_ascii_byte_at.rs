// Generated macro for wtf8_ascii_byte_at (function)
macro_rules! Depcrate_wtf8_testswtf8_ascii_byte_at {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_ascii_byte_at"}
// Dependencies: {}
# [test] fn wtf8_ascii_byte_at () { let slice = Wtf8 :: from_str ("aé 💩") ; assert_eq ! (slice . ascii_byte_at (0) , b'a') ; assert_eq ! (slice . ascii_byte_at (1) , b'\xFF') ; assert_eq ! (slice . ascii_byte_at (2) , b'\xFF') ; assert_eq ! (slice . ascii_byte_at (3) , b' ') ; assert_eq ! (slice . ascii_byte_at (4) , b'\xFF') ; }
};
}
