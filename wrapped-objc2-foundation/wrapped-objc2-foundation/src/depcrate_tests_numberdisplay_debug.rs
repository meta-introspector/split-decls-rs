// Generated macro for display_debug (function)
macro_rules! Depcrate_tests_numberdisplay_debug {
() => {
// Module: crate::tests::number
// Provides: {"display_debug"}
// Dependencies: {}
# [test] # [cfg (feature = "NSString")] fn display_debug () { use alloc :: format ; use core :: fmt ; fn assert_display_debug < T : fmt :: Debug + fmt :: Display > (val : T , expected : & str) { assert_eq ! (format ! ("{val}") , expected) ; assert_eq ! (format ! ("{val:?}") , expected) ; } assert_display_debug (NSNumber :: new_u8 (171) , "171") ; assert_display_debug (NSNumber :: new_i8 (- 12) , "-12") ; assert_display_debug (NSNumber :: new_u32 (0xdeadbeef) , "3735928559") ; assert_display_debug (NSNumber :: new_f32 (1.1) , "1.1") ; assert_display_debug (NSNumber :: new_f32 (1.0) , "1") ; assert_display_debug (NSNumber :: new_bool (true) , "1") ; assert_display_debug (NSNumber :: new_bool (false) , "0") ; }
};
}
