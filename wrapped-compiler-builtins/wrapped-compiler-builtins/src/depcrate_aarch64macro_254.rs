// Generated macro for macro_254 (macro)
macro_rules! Depcrate_aarch64macro_254 {
() => {
// Module: crate::aarch64
// Provides: {"macro_254"}
// Dependencies: {}
intrinsics ! { # [unsafe (naked)] # [cfg (target_os = "uefi")] pub unsafe extern "custom" fn __chkstk () { core :: arch :: naked_asm ! (".p2align 2" , "lsl    x16, x15, #4" , "mov    x17, sp" , "1:" , "sub    x17, x17, 4096" , "subs   x16, x16, 4096" , "ldr    xzr, [x17]" , "b.gt   1b" , "ret" ,) ; } }
};
}
