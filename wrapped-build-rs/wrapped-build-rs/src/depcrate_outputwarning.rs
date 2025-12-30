// Generated macro for warning (function)
macro_rules! Depcrate_outputwarning {
() => {
// Module: crate::output
// Provides: {"warning"}
// Dependencies: {}
# [doc = " The `warning` instruction tells Cargo to display a warning after the build"] # [doc = " script has finished running."] # [doc = ""] # [doc = " Warnings are only shown for path dependencies"] # [doc = " (that is, those you’re working on locally), so for example warnings printed"] # [doc = " out in [crates.io] crates are not emitted by default. The `-vv` “very verbose”"] # [doc = " flag may be used to have Cargo display warnings for all crates."] # [doc = ""] # [doc = " [crates.io]: https://crates.io/"] # [track_caller] pub fn warning (message : & str) { if message . contains ('\n') { panic ! ("cannot emit warning: message contains newline") ; } emit ("warning" , message) ; }
};
}
