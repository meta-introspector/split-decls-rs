// Generated macro for wtf8_encode_wide_size_hint (function)
macro_rules! Depcrate_wtf8_testswtf8_encode_wide_size_hint {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_encode_wide_size_hint"}
// Dependencies: {}
# [test] fn wtf8_encode_wide_size_hint () { let string = Wtf8Buf :: from_str ("\u{12345}") ; let mut iter = string . encode_wide () ; assert_eq ! ((1 , Some (8)) , iter . size_hint ()) ; iter . next () . unwrap () ; assert_eq ! ((1 , Some (1)) , iter . size_hint ()) ; iter . next () . unwrap () ; assert_eq ! ((0 , Some (0)) , iter . size_hint ()) ; assert ! (iter . next () . is_none ()) ; }
};
}
