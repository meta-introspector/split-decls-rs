// Generated macro for iokit_family_err (macro)
macro_rules! Depcrate_macrosiokit_family_err {
() => {
// Module: crate::macros
// Provides: {"iokit_family_err"}
// Dependencies: {}
macro_rules ! iokit_family_err { ($ sub : expr , $ return : expr) => { (err_system ! (0x38) | ($ sub as i32) | $ return) as i32 } ; }
};
}
