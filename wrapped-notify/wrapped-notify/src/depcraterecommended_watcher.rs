// Generated macro for recommended_watcher (function)
macro_rules! Depcraterecommended_watcher {
() => {
// Module: crate
// Provides: {"recommended_watcher"}
// Dependencies: {}
# [doc = " Convenience method for creating the [`RecommendedWatcher`] for the current platform."] pub fn recommended_watcher < F > (event_handler : F) -> Result < RecommendedWatcher > where F : EventHandler , { RecommendedWatcher :: new (event_handler , Config :: default ()) }
};
}
