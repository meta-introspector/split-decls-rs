// Generated macro for compare_and_swap_i128 (macro)
macro_rules! Depcrate_aarch64_linuxcompare_and_swap_i128 {
() => {
// Module: crate::aarch64_linux
// Provides: {"compare_and_swap_i128"}
// Dependencies: {}
macro_rules ! compare_and_swap_i128 { ($ ordering : ident , $ name : ident) => { intrinsics ! { # [maybe_use_optimized_c_shim] # [unsafe (naked)] pub unsafe extern "C" fn $ name (expected : i128 , desired : i128 , ptr : * mut i128) -> i128 { core :: arch :: naked_asm ! { try_lse_op ! ("cas" , $ ordering , 16 , 0 , 1 , 2 , 3 , [x4]) , "mov    x16, x0" , "mov    x17, x1" , "0:" , concat ! (ldxp ! ($ ordering) , " x0, x1, [x4]") , "cmp    x0, x16" , "ccmp   x1, x17, #0, eq" , "bne    1f" , concat ! (stxp ! ($ ordering) , " w15, x2, x3, [x4]") , "cbnz   w15, 0b" , "1:" , "ret" , have_lse = sym crate :: aarch64_linux :: HAVE_LSE_ATOMICS , } } } } ; }
};
}
