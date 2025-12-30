// Generated macro for impl_129 (impl)
macro_rules! Depcrate_log_stdout_loggerimpl_129 {
() => {
// Module: crate::log::stdout_logger
// Provides: {"impl_129"}
// Dependencies: {}
impl Log for StdoutLogger { fn enabled (& self , metadata : & Metadata) -> bool { (self . should_log) (metadata) } fn log (& self , record : & LogRecord) { if ! self . enabled (record . metadata ()) { return ; } match DefmtRecord :: new (record) { Some (record) => { let sink = io :: stdout () . lock () ; if record . level () . is_some () { self . print_defmt_record (record , sink) ; } else { self . print_defmt_record_without_format (record , sink) ; } } None => { let sink = io :: stderr () . lock () ; self . print_host_record (record , sink) ; } } } fn flush (& self) { } }
};
}
