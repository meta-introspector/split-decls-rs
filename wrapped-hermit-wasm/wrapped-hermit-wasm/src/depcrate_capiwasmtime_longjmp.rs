// Generated macro for wasmtime_longjmp (function)
macro_rules! Depcrate_capiwasmtime_longjmp {
() => {
// Module: crate::capi
// Provides: {"wasmtime_longjmp"}
// Dependencies: {}
# [doc = " Paired with `wasmtime_setjmp` this is used to jump back to the `setjmp`"] # [doc = " point."] # [doc = ""] # [doc = " The argument here was originally passed to `wasmtime_setjmp` through its"] # [doc = " out-param."] # [doc = ""] # [doc = " This function cannot return."] # [doc = ""] # [doc = " This function may be invoked from the `wasmtime_trap_handler_t`"] # [doc = " configured by `wasmtime_init_traps`."] # [unsafe (no_mangle)] pub extern "C" fn wasmtime_longjmp (jmp_buf : * const u8) -> ! { unsafe { longjmp (jmp_buf , 1) ; } }
};
}
