// Generated macro for trim_last_terminator (function)
macro_rules! Depcrate_ext_slicetrim_last_terminator {
() => {
// Module: crate::ext_slice
// Provides: {"trim_last_terminator"}
// Dependencies: {}
fn trim_last_terminator (mut s : & [u8]) -> & [u8] { if s . last_byte () == Some (b'\n') { s = & s [.. s . len () - 1] ; if s . last_byte () == Some (b'\r') { s = & s [.. s . len () - 1] ; } } s }
};
}
