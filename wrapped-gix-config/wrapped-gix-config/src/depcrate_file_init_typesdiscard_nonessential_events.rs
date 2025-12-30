// Generated macro for discard_nonessential_events (function)
macro_rules! Depcrate_file_init_typesdiscard_nonessential_events {
() => {
// Module: crate::file::init::types
// Provides: {"discard_nonessential_events"}
// Dependencies: {}
fn discard_nonessential_events (e : & Event < '_ >) -> bool { match e { Event :: Whitespace (_) | Event :: Comment (_) | Event :: Newline (_) => false , Event :: SectionHeader (_) | Event :: SectionValueName (_) | Event :: KeyValueSeparator | Event :: Value (_) | Event :: ValueNotDone (_) | Event :: ValueDone (_) => true , } }
};
}
