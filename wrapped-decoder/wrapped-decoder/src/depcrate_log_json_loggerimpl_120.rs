// Generated macro for impl_120 (impl)
macro_rules! Depcrate_log_json_loggerimpl_120 {
() => {
// Module: crate::log::json_logger
// Provides: {"impl_120"}
// Dependencies: {}
impl Log for JsonLogger { fn enabled (& self , metadata : & Metadata) -> bool { (self . should_log) (metadata) } fn log (& self , record : & Record) { if ! self . enabled (record . metadata ()) { return ; } if let Some (record) = DefmtRecord :: new (record) { let mut sink = io :: stdout () . lock () ; let host_timestamp = OffsetDateTime :: now_utc () . unix_timestamp_nanos () . min (i64 :: MAX as i128) as i64 ; serde_json :: to_writer (& mut sink , & create_json_frame (record , host_timestamp)) . ok () ; writeln ! (sink) . ok () ; } else { let sink = io :: stderr () . lock () ; self . host_logger . print_host_record (record , sink) ; } } fn flush (& self) { } }
};
}
