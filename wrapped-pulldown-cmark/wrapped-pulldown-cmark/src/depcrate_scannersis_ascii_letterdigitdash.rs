// Generated macro for is_ascii_letterdigitdash (function)
macro_rules! Depcrate_scannersis_ascii_letterdigitdash {
() => {
// Module: crate::scanners
// Provides: {"is_ascii_letterdigitdash"}
// Dependencies: {}
fn is_ascii_letterdigitdash (c : u8) -> bool { c == b'-' || is_ascii_alphanumeric (c) }
};
}
