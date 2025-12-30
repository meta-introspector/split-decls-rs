// Generated macro for trim_record (function)
macro_rules! Depcrate_iotrim_record {
() => {
// Module: crate::io
// Provides: {"trim_record"}
// Dependencies: {}
fn trim_record (record : & mut Vec < u8 > , terminator : u8) { if record . last_byte () == Some (terminator) { record . pop_byte () ; } }
};
}
