// Generated macro for impl_130 (impl)
macro_rules! Depcrate_log_stdout_loggerimpl_130 {
() => {
// Module: crate::log::stdout_logger
// Provides: {"impl_130"}
// Dependencies: {}
impl StdoutLogger { pub fn new (formatter : Formatter , host_formatter : HostFormatter , should_log : impl Fn (& Metadata) -> bool + Sync + Send + 'static ,) -> Box < Self > { Box :: new (Self :: new_unboxed (formatter , host_formatter , should_log)) } pub fn new_unboxed (formatter : Formatter , host_formatter : HostFormatter , should_log : impl Fn (& Metadata) -> bool + Sync + Send + 'static ,) -> Self { Self { formatter , host_formatter , should_log : Box :: new (should_log) , } } fn print_defmt_record (& self , record : DefmtRecord , mut sink : StdoutLock) { let s = self . formatter . format (& record) ; writeln ! (sink , "{s}") . ok () ; } pub (super) fn print_defmt_record_without_format (& self , record : DefmtRecord , mut sink : StdoutLock ,) { let s = record . args () . to_string () ; writeln ! (sink , "{s}") . ok () ; } pub (super) fn print_host_record (& self , record : & LogRecord , mut sink : StderrLock) { let s = self . host_formatter . format (record) ; writeln ! (sink , "{s}") . ok () ; } }
};
}
