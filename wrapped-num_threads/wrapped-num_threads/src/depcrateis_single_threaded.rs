// Generated macro for is_single_threaded (function)
macro_rules! Depcrateis_single_threaded {
() => {
// Module: crate
// Provides: {"is_single_threaded"}
// Dependencies: {}
# [doc = " Determine if the current process is single-threaded. Returns `None` if the number of threads"] # [doc = " cannot be determined."] pub fn is_single_threaded () -> Option < bool > { num_threads () . map (| n | n . get () == 1) }
};
}
