// Generated macro for iokit_bluetooth_err (macro)
macro_rules! Depcrate_macrosiokit_bluetooth_err {
() => {
// Module: crate::macros
// Provides: {"iokit_bluetooth_err"}
// Dependencies: {}
macro_rules ! iokit_bluetooth_err { ($ return : expr) => { (err_system ! (0x38) | err_sub ! (8) | $ return) as u32 } ; }
};
}
