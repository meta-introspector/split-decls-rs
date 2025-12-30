// Generated macro for printyo (macro)
macro_rules! Depcrate_reports_eventsprintyo {
() => {
// Module: crate::reports::events
// Provides: {"printyo"}
// Dependencies: {}
macro_rules ! printyo { ($ k : expr , $ value : expr , $ s : expr) => { { if let Some (v) = $ value { $ s += & format ! ("{}={}, " , $ k , v) ; } } } ; }
};
}
