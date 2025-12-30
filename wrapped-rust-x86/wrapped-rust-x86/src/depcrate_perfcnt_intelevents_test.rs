// Generated macro for events_test (function)
macro_rules! Depcrate_perfcnt_intelevents_test {
() => {
// Module: crate::perfcnt::intel
// Provides: {"events_test"}
// Dependencies: {}
# [test] fn events_test () { events () . map (| cc | { cc . get ("INST_RETIRED.ANY") . map (| p | { assert ! (p . event_name == "INST_RETIRED.ANY") ; }) ; }) ; }
};
}
