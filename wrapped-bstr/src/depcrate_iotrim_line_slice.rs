// Generated macro for trim_line_slice (function)
macro_rules! Depcrate_iotrim_line_slice {
() => {
// Module: crate::io
// Provides: {"trim_line_slice"}
// Dependencies: {}
fn trim_line_slice (mut line : & [u8]) -> & [u8] { if line . last_byte () == Some (b'\n') { line = & line [.. line . len () - 1] ; if line . last_byte () == Some (b'\r') { line = & line [.. line . len () - 1] ; } } line }
};
}
