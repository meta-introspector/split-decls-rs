// Generated macro for is_rust_hash (function)
macro_rules! Depcrate_legacyis_rust_hash {
() => {
// Module: crate::legacy
// Provides: {"is_rust_hash"}
// Dependencies: {}
fn is_rust_hash (s : & str) -> bool { s . starts_with ('h') && s [1 ..] . chars () . all (| c | c . is_digit (16)) }
};
}
