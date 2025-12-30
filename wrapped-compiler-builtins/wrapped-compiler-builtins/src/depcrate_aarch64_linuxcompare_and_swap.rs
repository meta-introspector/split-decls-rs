// Generated macro for compare_and_swap (macro)
macro_rules! Depcrate_aarch64_linuxcompare_and_swap {
() => {
// Module: crate::aarch64_linux
// Provides: {"compare_and_swap"}
// Dependencies: {}
# [doc = " See <https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicI8.html#method.compare_and_swap>."] macro_rules ! compare_and_swap { ($ ordering : ident , $ bytes : tt , $ name : ident) => { intrinsics ! { # [maybe_use_optimized_c_shim] # [unsafe (naked)] pub unsafe extern "C" fn $ name (expected : int_ty ! ($ bytes) , desired : int_ty ! ($ bytes) , ptr : * mut int_ty ! ($ bytes)) -> int_ty ! ($ bytes) { core :: arch :: naked_asm ! { try_lse_op ! ("cas" , $ ordering , $ bytes , 0 , 1 , [x2]) , concat ! (uxt ! ($ bytes) , " " , reg ! ($ bytes , 16) , ", " , reg ! ($ bytes , 0)) , "0:" , concat ! (ldxr ! ($ ordering , $ bytes) , " " , reg ! ($ bytes , 0) , ", [x2]") , concat ! ("cmp " , reg ! ($ bytes , 0) , ", " , reg ! ($ bytes , 16)) , "bne    1f" , concat ! (stxr ! ($ ordering , $ bytes) , " w17, " , reg ! ($ bytes , 1) , ", [x2]") , "cbnz   w17, 0b" , "1:" , "ret" , have_lse = sym crate :: aarch64_linux :: HAVE_LSE_ATOMICS , } } } } ; }
};
}
