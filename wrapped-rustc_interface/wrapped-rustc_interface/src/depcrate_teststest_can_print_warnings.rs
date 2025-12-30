// Generated macro for test_can_print_warnings (function)
macro_rules! Depcrate_teststest_can_print_warnings {
() => {
// Module: crate::tests
// Provides: {"test_can_print_warnings"}
// Dependencies: {}
# [test] fn test_can_print_warnings () { sess_and_cfg (& ["-Awarnings"] , | sess , _cfg | { assert ! (! sess . dcx () . can_emit_warnings ()) ; }) ; sess_and_cfg (& ["-Awarnings" , "-Dwarnings"] , | sess , _cfg | { assert ! (sess . dcx () . can_emit_warnings ()) ; }) ; sess_and_cfg (& ["-Adead_code"] , | sess , _cfg | { assert ! (sess . dcx () . can_emit_warnings ()) ; }) ; }
};
}
