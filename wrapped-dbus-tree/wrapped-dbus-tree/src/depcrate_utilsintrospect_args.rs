// Generated macro for introspect_args (function)
macro_rules! Depcrate_utilsintrospect_args {
() => {
// Module: crate::utils
// Provides: {"introspect_args"}
// Dependencies: {}
pub fn introspect_args (args : & [Argument] , indent : & str , dir : & str) -> String { args . iter () . fold ("" . to_string () , | aa , az | format ! ("{}{}" , aa , az . introspect (indent , dir))) }
};
}
