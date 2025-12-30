// Generated macro for run_ctors_once (function)
macro_rules! Depcrate_rtrun_ctors_once {
() => {
// Module: crate::rt
// Provides: {"run_ctors_once"}
// Dependencies: {}
# [doc = " Provide a hook for generated export functions to run static constructors at"] # [doc = " most once."] # [doc = ""] # [doc = " wit-bindgen-rust generates a call to this function at the start of all"] # [doc = " component export functions. Importantly, it is not called as part of"] # [doc = " `cabi_realloc`, which is a *core* export func, but should not execute ctors."] # [cfg (target_arch = "wasm32")] pub fn run_ctors_once () { static mut RUN : bool = false ; unsafe { if ! RUN { extern "C" { fn __wasm_call_ctors () ; } __wasm_call_ctors () ; RUN = true ; } } }
};
}
