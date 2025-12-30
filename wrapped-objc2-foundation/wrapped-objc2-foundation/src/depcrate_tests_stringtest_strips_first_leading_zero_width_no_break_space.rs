// Generated macro for test_strips_first_leading_zero_width_no_break_space (function)
macro_rules! Depcrate_tests_stringtest_strips_first_leading_zero_width_no_break_space {
() => {
// Module: crate::tests::string
// Provides: {"test_strips_first_leading_zero_width_no_break_space"}
// Dependencies: {}
# [test] # [doc = " Apparently NSString does this for some reason?"] fn test_strips_first_leading_zero_width_no_break_space () { let ns_string = NSString :: from_str ("\u{feff}") ; let expected = "" ; autoreleasepool (| pool | unsafe { assert_eq ! (ns_string . to_str (pool) , expected) ; }) ; assert_eq ! (ns_string . to_string () , expected) ; assert_eq ! (ns_string . len () , 0) ; let s = "\u{feff}\u{feff}a\u{feff}" ; let expected = if cfg ! (feature = "gnustep-1-7") { "a\u{feff}" } else { "\u{feff}a\u{feff}" } ; let ns_string = NSString :: from_str (s) ; autoreleasepool (| pool | unsafe { assert_eq ! (ns_string . to_str (pool) , expected) ; }) ; assert_eq ! (ns_string . to_string () , expected) ; assert_eq ! (ns_string . len () , expected . len ()) ; }
};
}
