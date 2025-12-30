// Generated macro for debug_reprs (function)
macro_rules! Depcrate_testerdebug_reprs {
() => {
// Module: crate::tester
// Provides: {"debug_reprs"}
// Dependencies: {}
# [doc = " Return a vector of the debug formatting of each item in `args`"] fn debug_reprs (args : & [& dyn Debug]) -> Vec < String > { args . iter () . map (| x | format ! ("{x:?}")) . collect () }
};
}
