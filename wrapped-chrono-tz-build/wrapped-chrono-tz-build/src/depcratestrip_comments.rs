// Generated macro for strip_comments (function)
macro_rules! Depcratestrip_comments {
() => {
// Module: crate
// Provides: {"strip_comments"}
// Dependencies: {}
fn strip_comments (mut line : String) -> String { if let Some (pos) = line . find ('#') { line . truncate (pos) ; } ; line }
};
}
