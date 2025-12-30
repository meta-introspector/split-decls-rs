// Generated macro for err_system (macro)
macro_rules! Depcrate_macroserr_system {
() => {
// Module: crate::macros
// Provides: {"err_system"}
// Dependencies: {}
macro_rules ! err_system { ($ x : expr) => { ((($ x as u32) & 0x3f) as i32) << 26 } ; }
};
}
