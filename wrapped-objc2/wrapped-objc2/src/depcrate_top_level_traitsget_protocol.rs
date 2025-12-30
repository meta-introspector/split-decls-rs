// Generated macro for get_protocol (function)
macro_rules! Depcrate_top_level_traitsget_protocol {
() => {
// Module: crate::top_level_traits
// Provides: {"get_protocol"}
// Dependencies: {}
fn get_protocol (name : & str) -> Option < & 'static AnyProtocol > { let name = CString :: new (name) . expect ("protocol name must be UTF-8") ; AnyProtocol :: get (& name) }
};
}
