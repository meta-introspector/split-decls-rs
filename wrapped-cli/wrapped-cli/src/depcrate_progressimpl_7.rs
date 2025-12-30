// Generated macro for impl_7 (impl)
macro_rules! Depcrate_progressimpl_7 {
() => {
// Module: crate::progress
// Provides: {"impl_7"}
// Dependencies: {}
impl < R : Read > ProgressMonitor < R > { # [doc = " Create a new progress monitor, initialized with zero bytes read"] pub fn new (reader : R , size : usize) -> Self { let style = ProgressStyle :: with_template ("{wide_bar} {binary_bytes}/{binary_total_bytes}  \n[est. {eta} remaining]" ,) . unwrap () ; let progress_bar = ProgressBar :: new (size as u64) . with_style (style) ; progress_bar . set_draw_target (ProgressDrawTarget :: stderr_with_hz (8)) ; Self { reader , total : size , read : 0 , progress_bar , } } # [doc = " This function is called whenever a new read is made, and is responsible for updating the UI"] fn update (& mut self , delta : u64) { self . progress_bar . inc (delta) ; if self . total == self . read && ! self . progress_bar . is_finished () { self . progress_bar . finish_and_clear () ; info ! ("processed {} in {} ({}/s avg)" , fmt_size (self . total as f64) , fmt_duration (self . progress_bar . elapsed ()) , fmt_size (self . total as f64 / self . progress_bar . elapsed () . as_secs_f64 ())) ; } } }
};
}
