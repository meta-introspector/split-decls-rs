// Generated macro for impl_121 (impl)
macro_rules! Depcrate_log_json_loggerimpl_121 {
() => {
// Module: crate::log::json_logger
// Provides: {"impl_121"}
// Dependencies: {}
impl JsonLogger { pub fn new (formatter : Formatter , host_formatter : HostFormatter , should_log : impl Fn (& Metadata) -> bool + Sync + Send + 'static ,) -> Box < Self > { Box :: new (Self { should_log : Box :: new (should_log) , host_logger : StdoutLogger :: new_unboxed (formatter , host_formatter , | _ | true) , }) } pub fn print_schema_version () { let mut sink = io :: stdout () . lock () ; serde_json :: to_writer (& mut sink , & SCHEMA_VERSION) . ok () ; writeln ! (sink) . ok () ; } }
};
}
