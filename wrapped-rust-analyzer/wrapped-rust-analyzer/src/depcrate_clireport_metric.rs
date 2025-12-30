// Generated macro for report_metric (function)
macro_rules! Depcrate_clireport_metric {
() => {
// Module: crate::cli
// Provides: {"report_metric"}
// Dependencies: {}
fn report_metric (metric : & str , value : u64 , unit : & str) { if std :: env :: var ("RA_METRICS") . is_err () { return ; } println ! ("METRIC:{metric}:{value}:{unit}") }
};
}
