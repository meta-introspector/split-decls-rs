// Generated macro for trim_ascii_end (function)
macro_rules! Depcrate_byte_recordtrim_ascii_end {
() => {
// Module: crate::byte_record
// Provides: {"trim_ascii_end"}
// Dependencies: {}
fn trim_ascii_end (mut bytes : & [u8]) -> & [u8] { while let [rest @ .. , last] = bytes { if last . is_ascii_whitespace () { bytes = rest ; } else { break ; } } bytes }
};
}
