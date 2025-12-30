// Generated macro for cmov_eq (macro)
macro_rules! Depcrate_x86cmov_eq {
() => {
// Module: crate::x86
// Provides: {"cmov_eq"}
// Dependencies: {}
macro_rules ! cmov_eq { ($ xor : expr , $ instruction : expr , $ lhs : expr , $ rhs : expr , $ condition : expr , $ dst : expr) => { let mut tmp = *$ dst as u16 ; unsafe { asm ! { $ xor , $ instruction , inout (reg) *$ lhs => _ , in (reg) *$ rhs , inlateout (reg) tmp , in (reg) $ condition as u16 , options (pure , nomem , nostack) , } ; } *$ dst = tmp as u8 ; } ; }
};
}
