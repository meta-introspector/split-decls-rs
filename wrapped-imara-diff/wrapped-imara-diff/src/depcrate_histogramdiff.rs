// Generated macro for diff (function)
macro_rules! Depcrate_histogramdiff {
() => {
// Module: crate::histogram
// Provides: {"diff"}
// Dependencies: {}
pub fn diff (before : & [Token] , after : & [Token] , removed : & mut [bool] , added : & mut [bool] , num_tokens : u32 ,) { let mut histogram = Histogram :: new (num_tokens) ; histogram . run (before , after , removed , added) ; }
};
}
