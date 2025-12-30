// Generated macro for fetch_op (macro)
macro_rules! Depcrate_aarch64_linuxfetch_op {
() => {
// Module: crate::aarch64_linux
// Provides: {"fetch_op"}
// Dependencies: {}
# [doc = " See (e.g.) <https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicI8.html#method.fetch_add>."] macro_rules ! fetch_op { ($ ordering : ident , $ bytes : tt , $ name : ident , $ op : literal , $ lse_op : literal) => { intrinsics ! { # [maybe_use_optimized_c_shim] # [unsafe (naked)] pub unsafe extern "C" fn $ name (val : int_ty ! ($ bytes) , ptr : * mut int_ty ! ($ bytes)) -> int_ty ! ($ bytes) { core :: arch :: naked_asm ! { try_lse_op ! ($ lse_op , $ ordering , $ bytes , 0 , 0 , [x1]) , concat ! ("mov " , reg ! ($ bytes , 16) , ", " , reg ! ($ bytes , 0)) , "0:" , concat ! (ldxr ! ($ ordering , $ bytes) , " " , reg ! ($ bytes , 0) , ", [x1]") , concat ! ($ op , " " , reg ! ($ bytes , 17) , ", " , reg ! ($ bytes , 0) , ", " , reg ! ($ bytes , 16)) , concat ! (stxr ! ($ ordering , $ bytes) , " w15, " , reg ! ($ bytes , 17) , ", [x1]") , "cbnz  w15, 0b" , "ret" , have_lse = sym crate :: aarch64_linux :: HAVE_LSE_ATOMICS , } } } } }
};
}
