// Generated macro for wasmtime_setjmp (function)
macro_rules! Depcrate_capiwasmtime_setjmp {
() => {
// Module: crate::capi
// Provides: {"wasmtime_setjmp"}
// Dependencies: {}
# [doc = " Used to setup a frame on the stack to longjmp back to in the future."] # [doc = ""] # [doc = " This function is used for handling traps in WebAssembly and is paried"] # [doc = " with `wasmtime_longjmp`."] # [doc = ""] # [doc = " * `jmp_buf` - this argument is filled in with a pointer which if used"] # [doc = "   will be passed to `wasmtime_longjmp` later on by the runtime."] # [doc = " * `callback` - this callback should be invoked after `jmp_buf` is"] # [doc = "   configured."] # [doc = " * `payload` and `callee` - the two arguments to pass to `callback`."] # [doc = ""] # [doc = " Returns 0 if `wasmtime_longjmp` was used to return to this function."] # [doc = " Returns 1 if `wasmtime_longjmp` was not called and `callback` returned."] # [unsafe (no_mangle)] pub extern "C" fn wasmtime_setjmp (jmp_buf : * mut * const u8 , callback : extern "C" fn (* mut u8 , * mut u8) , payload : * mut u8 , callee : * mut u8 ,) -> i32 { cfg_if :: cfg_if ! { if # [cfg (target_arch = "aarch64")] { const BUF_SIZE : usize = 176 ; } else if # [cfg (target_arch = "x86_64")] { const BUF_SIZE : usize = 64 ; } else if # [cfg (target_arch = "riscv64")] { const BUF_SIZE : usize = 208 ; } } let buf : [u8 ; BUF_SIZE] = [0 ; BUF_SIZE] ; unsafe { if setjmp (buf . as_ptr ()) != 0 { return 0 ; } * jmp_buf = buf . as_ptr () ; } callback (payload , callee) ; 1 }
};
}
