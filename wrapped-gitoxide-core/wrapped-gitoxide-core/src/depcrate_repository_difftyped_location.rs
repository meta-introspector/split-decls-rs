// Generated macro for typed_location (function)
macro_rules! Depcrate_repository_difftyped_location {
() => {
// Module: crate::repository::diff
// Provides: {"typed_location"}
// Dependencies: {}
fn typed_location (mut location : BString , mode : EntryMode) -> BString { if mode . is_tree () { location . push (b'/') ; } location }
};
}
