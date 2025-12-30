// Generated macro for is_hex_literal (function)
macro_rules! Depcrate_deis_hex_literal {
() => {
// Module: crate::de
// Provides: {"is_hex_literal"}
// Dependencies: {}
fn is_hex_literal (s : & str) -> bool { s . len () > 2 && (& s [.. 2] == "0x" || & s [.. 2] == "0X") }
};
}
