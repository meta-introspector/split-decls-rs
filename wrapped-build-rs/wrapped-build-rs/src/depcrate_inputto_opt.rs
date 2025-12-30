// Generated macro for to_opt (function)
macro_rules! Depcrate_inputto_opt {
() => {
// Module: crate::input
// Provides: {"to_opt"}
// Dependencies: {}
fn to_opt (value : std :: ffi :: OsString) -> Option < std :: ffi :: OsString > { (! value . is_empty ()) . then_some (value) }
};
}
