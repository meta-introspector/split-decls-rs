// Generated macro for impl_2244 (impl)
macro_rules! Depcrate_mysql_connection_bindimpl_2244 {
() => {
// Module: crate::mysql::connection::bind
// Provides: {"impl_2244"}
// Dependencies: {}
impl From < u32 > for Flags { fn from (flags : u32) -> Self { Flags :: from_bits (flags) . expect ("We encountered an unknown type flag while parsing \
             Mysql's type information. If you see this error message \
             please open an issue at diesels github page." ,) } }
};
}
