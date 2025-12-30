// Generated macro for test_from_utf16 (function)
macro_rules! Depcrate_teststest_from_utf16 {
() => {
// Module: crate::tests
// Provides: {"test_from_utf16"}
// Dependencies: {}
# [test] fn test_from_utf16 () { let control = String :: from ("🦄 hello world! 🎮 ") ; let utf16_buf : Vec < u16 > = control . encode_utf16 () . collect () ; let compact = CompactString :: from_utf16 (utf16_buf) . unwrap () ; assert_eq ! (compact , control) ; cfg_if :: cfg_if ! { if # [cfg (target_pointer_width = "64")] { assert ! (! compact . is_heap_allocated ()) ; } else if # [cfg (target_pointer_width = "32")] { assert ! (compact . is_heap_allocated ()) ; } else { compile_error ! ("unsupported pointer width!") ; } } }
};
}
