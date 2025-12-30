// Generated macro for DebounceDataInner (struct)
macro_rules! DepcrateDebounceDataInner {
() => {
// Module: crate
// Provides: {"DebounceDataInner"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct DebounceDataInner < T > { queues : HashMap < PathBuf , Queue > , roots : Vec < (PathBuf , RecursiveMode) > , cache : T , rename_event : Option < (DebouncedEvent , Option < FileId >) > , rescan_event : Option < DebouncedEvent > , errors : Vec < Error > , timeout : Duration , }
};
}
