// Generated macro for ensure_percentages_less_than_100 (function)
macro_rules! Depcrate_tableensure_percentages_less_than_100 {
() => {
// Module: crate::table
// Provides: {"ensure_percentages_less_than_100"}
// Dependencies: {}
fn ensure_percentages_less_than_100 (widths : & [Constraint]) { for w in widths { if let Constraint :: Percentage (p) = w { assert ! (* p <= 100 , "Percentages should be between 0 and 100 inclusively.") ; } } }
};
}
