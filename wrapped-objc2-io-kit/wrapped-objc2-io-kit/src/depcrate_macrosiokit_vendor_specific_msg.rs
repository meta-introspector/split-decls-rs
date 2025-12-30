// Generated macro for iokit_vendor_specific_msg (macro)
macro_rules! Depcrate_macrosiokit_vendor_specific_msg {
() => {
// Module: crate::macros
// Provides: {"iokit_vendor_specific_msg"}
// Dependencies: {}
macro_rules ! iokit_vendor_specific_msg { ($ message : expr) => { (err_system ! (0x38) | err_sub ! (- 2) | $ message) as u32 } ; }
};
}
