// Generated macro for is_symbol_like (function)
macro_rules! Depcrateis_symbol_like {
() => {
// Module: crate
// Provides: {"is_symbol_like"}
// Dependencies: {}
fn is_symbol_like (s : & str) -> bool { s . chars () . all (| c | { is_ascii_alphanumeric (c) || is_ascii_punctuation (c) }) }
};
}
