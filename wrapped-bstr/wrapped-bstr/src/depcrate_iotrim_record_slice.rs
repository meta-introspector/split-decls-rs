// Generated macro for trim_record_slice (function)
macro_rules! Depcrate_iotrim_record_slice {
() => {
// Module: crate::io
// Provides: {"trim_record_slice"}
// Dependencies: {}
fn trim_record_slice (mut record : & [u8] , terminator : u8) -> & [u8] { if record . last_byte () == Some (terminator) { record = & record [.. record . len () - 1] ; } record }
};
}
