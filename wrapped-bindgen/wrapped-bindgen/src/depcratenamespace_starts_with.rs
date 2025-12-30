// Generated macro for namespace_starts_with (function)
macro_rules! Depcratenamespace_starts_with {
() => {
// Module: crate
// Provides: {"namespace_starts_with"}
// Dependencies: {}
fn namespace_starts_with (namespace : & str , starts_with : & str) -> bool { namespace . starts_with (starts_with) && (namespace . len () == starts_with . len () || namespace . as_bytes () . get (starts_with . len ()) == Some (& b'.')) }
};
}
