// Generated macro for trim_line (function)
macro_rules! Depcrate_iotrim_line {
() => {
// Module: crate::io
// Provides: {"trim_line"}
// Dependencies: {}
fn trim_line (line : & mut Vec < u8 >) { if line . last_byte () == Some (b'\n') { line . pop_byte () ; if line . last_byte () == Some (b'\r') { line . pop_byte () ; } } }
};
}
