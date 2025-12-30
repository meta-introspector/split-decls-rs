// Generated macro for valid_value (function)
macro_rules! Depcrate_mimevalid_value {
() => {
// Module: crate::mime
// Provides: {"valid_value"}
// Dependencies: {}
fn valid_value (s : & str) -> bool { s . chars () . all (| c | { matches ! (c , '\t' | ' ' ..='~' | '\u{80}' ..='\u{FF}') }) }
};
}
