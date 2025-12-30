// Generated macro for new_debouncer (function)
macro_rules! Depcratenew_debouncer {
() => {
// Module: crate
// Provides: {"new_debouncer"}
// Dependencies: {}
# [doc = " Short function to create a new debounced watcher with the recommended debouncer and the built-in file ID cache."] # [doc = ""] # [doc = " Timeout is the amount of time after which a debounced event is emitted."] # [doc = ""] # [doc = " If `tick_rate` is `None`, notify will select a tick rate that is 1/4 of the provided timeout."] pub fn new_debouncer < F : DebounceEventHandler > (timeout : Duration , tick_rate : Option < Duration > , event_handler : F ,) -> Result < Debouncer < RecommendedWatcher , RecommendedCache > , Error > { new_debouncer_opt :: < F , RecommendedWatcher , RecommendedCache > (timeout , tick_rate , event_handler , RecommendedCache :: new () , notify :: Config :: default () ,) }
};
}
