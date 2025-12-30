// Generated macro for csel (macro)
macro_rules! Depcrate_aarch64csel {
() => {
// Module: crate::aarch64
// Provides: {"csel"}
// Dependencies: {}
macro_rules ! csel { ($ csel : expr , $ dst : expr , $ src : expr , $ condition : expr) => { unsafe { asm ! { "cmp {0:w}, 0" , $ csel , in (reg) $ condition , inlateout (reg) *$ dst , in (reg) *$ src , in (reg) *$ dst , options (pure , nomem , nostack) , } ; } } ; }
};
}
