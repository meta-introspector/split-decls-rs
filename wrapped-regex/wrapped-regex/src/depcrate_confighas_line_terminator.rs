// Generated macro for has_line_terminator (function)
macro_rules! Depcrate_confighas_line_terminator {
() => {
// Module: crate::config
// Provides: {"has_line_terminator"}
// Dependencies: {}
# [doc = " Returns true if the given literal string contains any byte from the line"] # [doc = " terminator given."] fn has_line_terminator (lineterm : LineTerminator , literal : & str) -> bool { if lineterm . is_crlf () { literal . as_bytes () . iter () . copied () . any (| b | b == b'\r' || b == b'\n') } else { literal . as_bytes () . iter () . copied () . any (| b | b == lineterm . as_byte ()) } }
};
}
