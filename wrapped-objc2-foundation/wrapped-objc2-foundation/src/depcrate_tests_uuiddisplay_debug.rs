// Generated macro for display_debug (function)
macro_rules! Depcrate_tests_uuiddisplay_debug {
() => {
// Module: crate::tests::uuid
// Provides: {"display_debug"}
// Dependencies: {}
# [test] # [cfg (feature = "NSString")] # [ignore = "encoding depends on Foundation version"] fn display_debug () { let uuid = NSUUID :: from_bytes ([10 ; 16]) ; let expected = "0A0A0A0A-0A0A-0A0A-0A0A-0A0A0A0A0A0A" ; assert_eq ! (format ! ("{uuid}") , expected) ; assert_eq ! (format ! ("{uuid:?}") , expected) ; }
};
}
