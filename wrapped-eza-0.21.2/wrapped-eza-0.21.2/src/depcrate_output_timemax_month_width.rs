// Generated macro for MAX_MONTH_WIDTH (static)
macro_rules! Depcrate_output_timeMAX_MONTH_WIDTH {
() => {
// Module: crate::output::time
// Provides: {"MAX_MONTH_WIDTH"}
// Dependencies: {}
static MAX_MONTH_WIDTH : LazyLock < usize > = LazyLock :: new (| | { (0 .. 11) . map (| i | UnicodeWidthStr :: width (& * LOCALE . short_month_name (i))) . max () . unwrap () }) ;
};
}
