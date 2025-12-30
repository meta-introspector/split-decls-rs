// Generated macro for potentially_plural_count (function)
macro_rules! Depcrate_checkpotentially_plural_count {
() => {
// Module: crate::check
// Provides: {"potentially_plural_count"}
// Dependencies: {}
pub fn potentially_plural_count (count : usize , word : & str) -> String { format ! ("{} {}{}" , count , word , pluralize ! (count)) }
};
}
