// Generated macro for iokit_common_msg (macro)
macro_rules! Depcrate_macrosiokit_common_msg {
() => {
// Module: crate::macros
// Provides: {"iokit_common_msg"}
// Dependencies: {}
macro_rules ! iokit_common_msg { ($ message : expr) => { (err_system ! (0x38) | err_sub ! (0) | $ message) as u32 } ; }
};
}
