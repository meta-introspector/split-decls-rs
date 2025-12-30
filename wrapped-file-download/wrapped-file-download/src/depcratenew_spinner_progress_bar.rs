// Generated macro for new_spinner_progress_bar (function)
macro_rules! Depcratenew_spinner_progress_bar {
() => {
// Module: crate
// Provides: {"new_spinner_progress_bar"}
// Dependencies: {}
# [doc = " Creates a new process bar for processing that will take an unknown amount of time"] fn new_spinner_progress_bar () -> ProgressBar { let progress_bar = ProgressBar :: new (42) ; progress_bar . set_style (ProgressStyle :: default_spinner () . template ("{spinner:.green} {wide_msg}") . expect ("ProgresStyle::template direct input to be correct") ,) ; progress_bar . enable_steady_tick (Duration :: from_millis (100)) ; progress_bar }
};
}
