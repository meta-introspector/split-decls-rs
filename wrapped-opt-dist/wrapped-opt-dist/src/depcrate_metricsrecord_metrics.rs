// Generated macro for record_metrics (function)
macro_rules! Depcrate_metricsrecord_metrics {
() => {
// Module: crate::metrics
// Provides: {"record_metrics"}
// Dependencies: {}
# [doc = " Logs the individual metrics in a table and add Rustc and LLVM durations to the passed"] # [doc = " timer."] pub fn record_metrics (metrics : & BuildStep , timer : & mut TimerSection) { let llvm_steps = metrics . find_all_by_type ("bootstrap::llvm::Llvm") ; let llvm_duration : Duration = llvm_steps . into_iter () . map (| s | s . duration) . sum () ; let rustc_steps = metrics . find_all_by_type ("bootstrap::compile::Rustc") ; let rustc_duration : Duration = rustc_steps . into_iter () . map (| s | s . duration) . sum () ; let rustc_duration = rustc_duration . saturating_sub (llvm_duration) ; if ! llvm_duration . is_zero () { timer . add_duration ("LLVM" , llvm_duration) ; } if ! rustc_duration . is_zero () { timer . add_duration ("Rustc" , rustc_duration) ; } let output = format_build_steps (metrics) ; log :: info ! ("Build step durations\n{output}") ; }
};
}
