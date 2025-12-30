// Generated macro for printy_json (macro)
macro_rules! Depcrate_reports_eventsprinty_json {
() => {
// Module: crate::reports::events
// Provides: {"printy_json"}
// Dependencies: {}
macro_rules ! printy_json { ($ k : expr , $ value : expr , $ s : expr) => { { $ s += & format ! ("{}={}, " , $ k , & serde_json :: to_string (&$ value) . unwrap () . replace ("\"" , "")) ; } } ; }
};
}
