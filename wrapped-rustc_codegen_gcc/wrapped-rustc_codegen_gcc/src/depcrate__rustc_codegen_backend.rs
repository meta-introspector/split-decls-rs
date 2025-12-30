// Generated macro for __rustc_codegen_backend (function)
macro_rules! Depcrate__rustc_codegen_backend {
() => {
// Module: crate
// Provides: {"__rustc_codegen_backend"}
// Dependencies: {}
# [doc = " This is the entrypoint for a hot plugged rustc_codegen_gccjit"] # [unsafe (no_mangle)] pub fn __rustc_codegen_backend () -> Box < dyn CodegenBackend > { # [cfg (feature = "master")] let info = { let context = Context :: default () ; Arc :: new (Mutex :: new (IntoDynSyncSend (context . get_target_info ()))) } ; # [cfg (not (feature = "master"))] let info = Arc :: new (Mutex :: new (IntoDynSyncSend (TargetInfo { supports_128bit_integers : AtomicBool :: new (false) , }))) ; Box :: new (GccCodegenBackend { target_info : LockedTargetInfo { info } }) }
};
}
