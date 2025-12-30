// Generated macro for swap (macro)
macro_rules! Depcrate_aarch64_linuxswap {
() => {
// Module: crate::aarch64_linux
// Provides: {"swap"}
// Dependencies: {}
# [doc = " See <https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicI8.html#method.swap>."] macro_rules ! swap { ($ ordering : ident , $ bytes : tt , $ name : ident) => { intrinsics ! { # [maybe_use_optimized_c_shim] # [unsafe (naked)] pub unsafe extern "C" fn $ name (left : int_ty ! ($ bytes) , right_ptr : * mut int_ty ! ($ bytes)) -> int_ty ! ($ bytes) { core :: arch :: naked_asm ! { try_lse_op ! ("swp" , $ ordering , $ bytes , 0 , 0 , [x1]) , concat ! ("mov " , reg ! ($ bytes , 16) , ", " , reg ! ($ bytes , 0)) , "0:" , concat ! (ldxr ! ($ ordering , $ bytes) , " " , reg ! ($ bytes , 0) , ", [x1]") , concat ! (stxr ! ($ ordering , $ bytes) , " w17, " , reg ! ($ bytes , 16) , ", [x1]") , "cbnz   w17, 0b" , "ret" , have_lse = sym crate :: aarch64_linux :: HAVE_LSE_ATOMICS , } } } } ; }
};
}
