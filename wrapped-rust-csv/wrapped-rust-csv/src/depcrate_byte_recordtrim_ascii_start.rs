// Generated macro for trim_ascii_start (function)
macro_rules! Depcrate_byte_recordtrim_ascii_start {
() => {
// Module: crate::byte_record
// Provides: {"trim_ascii_start"}
// Dependencies: {}
fn trim_ascii_start (mut bytes : & [u8]) -> & [u8] { while let [first , rest @ ..] = bytes { if first . is_ascii_whitespace () { bytes = rest ; } else { break ; } } bytes }
};
}
