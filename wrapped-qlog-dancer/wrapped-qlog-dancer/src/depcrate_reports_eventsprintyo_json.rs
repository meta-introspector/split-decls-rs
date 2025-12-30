// Generated macro for printyo_json (macro)
macro_rules! Depcrate_reports_eventsprintyo_json {
() => {
// Module: crate::reports::events
// Provides: {"printyo_json"}
// Dependencies: {}
macro_rules ! printyo_json { ($ k : expr , $ value : expr , $ s : expr) => { { if let Some (v) = $ value { $ s += & format ! ("{}={}, " , $ k , & serde_json :: to_string (& v) . unwrap () . replace ("\"" , "")) ; } } } ; }
};
}
