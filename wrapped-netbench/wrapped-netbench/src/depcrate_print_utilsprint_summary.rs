// Generated macro for print_summary (function)
macro_rules! Depcrate_print_utilsprint_summary {
() => {
// Module: crate::print_utils
// Provides: {"print_summary"}
// Dependencies: {}
# [doc = " Nicely outputs summary of execution with stats and CDF points."] pub fn print_summary (hist : hdrhist :: HDRHist) { println ! ("Sent/received everything!") ; print_line () ; println ! ("HDRHIST summary, measure in ns") ; print_line () ; println ! ("summary:\n{:#?}" , hist . summary () . collect ::< Vec < _ >> ()) ; print_line () ; println ! ("Summary_string:\n{}" , hist . summary_string ()) ; print_line () ; println ! ("CDF summary:\n") ; for entry in hist . ccdf_upper_bound () { println ! ("{entry:?}") ; } }
};
}
