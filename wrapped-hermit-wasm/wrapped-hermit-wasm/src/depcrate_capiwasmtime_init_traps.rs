// Generated macro for wasmtime_init_traps (function)
macro_rules! Depcrate_capiwasmtime_init_traps {
() => {
// Module: crate::capi
// Provides: {"wasmtime_init_traps"}
// Dependencies: {}
# [doc = " Initializes trap-handling logic for this platform."] # [doc = ""] # [doc = " Wasmtime's implementation of WebAssembly relies on the ability to catch"] # [doc = " signals/traps/etc. For example divide-by-zero may raise a machine"] # [doc = " exception. Out-of-bounds memory accesses may also raise a machine"] # [doc = " exception. This function is used to initialize trap handling."] # [doc = ""] # [doc = " The `handler` provided is a function pointer to invoke whenever a trap"] # [doc = " is encountered. The `handler` is invoked whenever a trap is caught by"] # [doc = " the system."] # [doc = ""] # [doc = " Returns 0 on success and an error code on failure."] # [unsafe (no_mangle)] pub extern "C" fn wasmtime_init_traps (_handler : wasmtime_trap_handler_t) -> i32 { 0 }
};
}
