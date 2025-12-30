// Generated macro for test (module)
macro_rules! Depcrate_output_timetest {
() => {
// Module: crate::output::time
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn short_month_width_japanese () { let max_month_width = 4 ; let month = "1\u{2F49}" ; let padding = short_month_padding (max_month_width , month) ; let final_str = format ! ("{:<width$}" , month , width = padding) ; assert_eq ! (max_month_width , UnicodeWidthStr :: width (final_str . as_str ())) ; } # [test] fn short_month_width_hindi () { let max_month_width = 4 ; assert_eq ! (true , ["\u{091C}\u{0928}\u{0970}" , "\u{092B}\u{093C}\u{0930}\u{0970}" , "\u{092E}\u{093E}\u{0930}\u{094D}\u{091A}" , "\u{0905}\u{092A}\u{094D}\u{0930}\u{0948}\u{0932}" , "\u{092E}\u{0908}" , "\u{091C}\u{0942}\u{0928}" , "\u{091C}\u{0941}\u{0932}\u{0970}" , "\u{0905}\u{0917}\u{0970}" , "\u{0938}\u{093F}\u{0924}\u{0970}" , "\u{0905}\u{0915}\u{094D}\u{0924}\u{0942}\u{0970}" , "\u{0928}\u{0935}\u{0970}" , "\u{0926}\u{093F}\u{0938}\u{0970}" ,] . iter () . map (| month | format ! ("{:<width$}" , month , width = short_month_padding (max_month_width , month))) . all (| string | UnicodeWidthStr :: width (string . as_str ()) == max_month_width)) ; } }
};
}
