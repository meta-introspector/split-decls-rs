// Generated macro for printy (macro)
macro_rules! Depcrate_reports_eventsprinty {
() => {
// Module: crate::reports::events
// Provides: {"printy"}
// Dependencies: {}
macro_rules ! printy { ($ k : expr , $ value : expr , $ s : expr) => { { $ s += & format ! ("{}={}, " , $ k , $ value) ; } } ; }
};
}
