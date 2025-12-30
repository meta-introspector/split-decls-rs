// Generated macro for impl_89 (impl)
macro_rules! Depcrate_log_formatimpl_89 {
() => {
// Module: crate::log::format
// Provides: {"impl_89"}
// Dependencies: {}
impl HostFormatter { # [doc = " Create a new host formatter using the given config"] pub fn new (config : FormatterConfig) -> Self { Self { formatter : InternalFormatter :: new (config , Source :: Host) , } } # [doc = " Format the given [`log::Record`]."] pub fn format (& self , record : & LogRecord) -> String { self . formatter . format (& Record :: Host (record)) } }
};
}
