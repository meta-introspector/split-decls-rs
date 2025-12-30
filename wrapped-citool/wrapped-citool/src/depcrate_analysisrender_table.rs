// Generated macro for render_table (function)
macro_rules! Depcrate_analysisrender_table {
() => {
// Module: crate::analysis
// Provides: {"render_table"}
// Dependencies: {}
fn render_table (suites : BTreeMap < String , TestSuiteRecord >) -> String { use std :: fmt :: Write ; let mut table = "| Test suite | Passed ✅ | Ignored 🚫 | Failed  ❌ |\n" . to_string () ; writeln ! (table , "|:------|------:|------:|------:|") . unwrap () ; fn compute_pct (value : f64 , total : f64) -> f64 { if total == 0.0 { 0.0 } else { value / total } } fn write_row (buffer : & mut String , name : & str , record : & TestSuiteRecord , surround : & str ,) -> std :: fmt :: Result { let TestSuiteRecord { passed , ignored , failed } = record ; let total = (record . passed + record . ignored + record . failed) as f64 ; let passed_pct = compute_pct (* passed as f64 , total) * 100.0 ; let ignored_pct = compute_pct (* ignored as f64 , total) * 100.0 ; let failed_pct = compute_pct (* failed as f64 , total) * 100.0 ; write ! (buffer , "| {surround}{name}{surround} |") ? ; write ! (buffer , " {surround}{passed} ({passed_pct:.0}%){surround} |") ? ; write ! (buffer , " {surround}{ignored} ({ignored_pct:.0}%){surround} |") ? ; writeln ! (buffer , " {surround}{failed} ({failed_pct:.0}%){surround} |") ? ; Ok (()) } let mut total = TestSuiteRecord :: default () ; for (name , record) in suites { write_row (& mut table , & name , & record , "") . unwrap () ; total . passed += record . passed ; total . ignored += record . ignored ; total . failed += record . failed ; } write_row (& mut table , "Total" , & total , "**") . unwrap () ; table }
};
}
