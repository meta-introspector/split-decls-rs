// Generated macro for cmov (macro)
macro_rules! Depcrate_x86cmov {
() => {
// Module: crate::x86
// Provides: {"cmov"}
// Dependencies: {}
macro_rules ! cmov { ($ instruction : expr , $ dst : expr , $ src : expr , $ condition : expr) => { unsafe { asm ! { "test {0}, {0}" , $ instruction , in (reg_byte) $ condition , inlateout (reg) *$ dst , in (reg) *$ src , options (pure , nomem , nostack) , } ; } } ; }
};
}
