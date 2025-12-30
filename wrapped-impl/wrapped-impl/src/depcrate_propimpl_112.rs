// Generated macro for impl_112 (impl)
macro_rules! Depcrate_propimpl_112 {
() => {
// Module: crate::prop
// Provides: {"impl_112"}
// Dependencies: {}
impl Variant < '_ > { pub (crate) fn from_field (& self) -> Option < & Field > { from_field (& self . fields) } pub (crate) fn source_field (& self) -> Option < & Field > { source_field (& self . fields) } pub (crate) fn backtrace_field (& self) -> Option < & Field > { backtrace_field (& self . fields) } pub (crate) fn distinct_backtrace_field (& self) -> Option < & Field > { let backtrace_field = self . backtrace_field () ? ; distinct_backtrace_field (backtrace_field , self . from_field ()) } }
};
}
