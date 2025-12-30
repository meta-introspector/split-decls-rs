// Generated macro for csel_eq (macro)
macro_rules! Depcrate_aarch64csel_eq {
() => {
// Module: crate::aarch64
// Provides: {"csel_eq"}
// Dependencies: {}
macro_rules ! csel_eq { ($ instruction : expr , $ lhs : expr , $ rhs : expr , $ condition : expr , $ dst : expr) => { let mut tmp = *$ dst as u16 ; unsafe { asm ! { "eor {0:w}, {1:w}, {2:w}" , "cmp {0:w}, 0" , $ instruction , out (reg) _ , in (reg) *$ lhs , in (reg) *$ rhs , inlateout (reg) tmp , in (reg) $ condition as u16 , in (reg) tmp , options (pure , nomem , nostack) , } ; } ; *$ dst = tmp as u8 ; } ; }
};
}
