// Generated macro for fdt_args (function)
macro_rules! Depcrate_envfdt_args {
() => {
// Module: crate::env
// Provides: {"fdt_args"}
// Dependencies: {}
pub fn fdt_args () -> Option < & 'static str > { fdt () . and_then (| fdt | fdt . chosen () . bootargs ()) }
};
}
