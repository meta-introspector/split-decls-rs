// Generated macro for iokit_family_msg (macro)
macro_rules! Depcrate_macrosiokit_family_msg {
() => {
// Module: crate::macros
// Provides: {"iokit_family_msg"}
// Dependencies: {}
macro_rules ! iokit_family_msg { ($ sub : expr , $ message : expr) => { (err_system ! (0x38) | ($ sub as i32) | $ message) as u32 } ; }
};
}
