// Generated macro for new_debouncer (function)
macro_rules! Depcratenew_debouncer {
() => {
// Module: crate
// Provides: {"new_debouncer"}
// Dependencies: {}
# [doc = " Short function to create a new debounced watcher with the recommended debouncer."] # [doc = ""] # [doc = " Timeout is the amount of time after which a debounced event is emitted or a continuous event is send, if there still are events incoming for the specific path."] pub fn new_debouncer < F : DebounceEventHandler > (timeout : Duration , event_handler : F ,) -> Result < Debouncer < RecommendedWatcher > , Error > { let config = Config :: default () . with_timeout (timeout) ; new_debouncer_opt :: < F , RecommendedWatcher > (config , event_handler) }
};
}
