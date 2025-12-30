// Generated macro for iokit_usb_msg (macro)
macro_rules! Depcrate_macrosiokit_usb_msg {
() => {
// Module: crate::macros
// Provides: {"iokit_usb_msg"}
// Dependencies: {}
macro_rules ! iokit_usb_msg { ($ message : expr) => { (err_system ! (0x38) | err_sub ! (1) | $ message) as u32 } ; }
};
}
