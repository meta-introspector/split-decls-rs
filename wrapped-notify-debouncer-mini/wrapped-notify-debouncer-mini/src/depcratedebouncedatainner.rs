// Generated macro for DebounceDataInner (struct)
macro_rules! DepcrateDebounceDataInner {
() => {
// Module: crate
// Provides: {"DebounceDataInner"}
// Dependencies: {}
struct DebounceDataInner { # [doc = " Path -> Event data"] event_map : HashMap < PathBuf , EventData > , # [doc = " timeout used to compare all events against, config"] timeout : Duration , # [doc = " Whether to time events exactly, or batch multiple together."] # [doc = " This reduces the amount of updates but possibly waiting longer than necessary for some events"] batch_mode : bool , # [doc = " next debounce deadline"] debounce_deadline : Option < Instant > , }
};
}
