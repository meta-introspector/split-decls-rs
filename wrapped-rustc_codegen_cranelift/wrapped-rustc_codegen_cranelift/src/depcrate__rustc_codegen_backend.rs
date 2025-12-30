// Generated macro for __rustc_codegen_backend (function)
macro_rules! Depcrate__rustc_codegen_backend {
() => {
// Module: crate
// Provides: {"__rustc_codegen_backend"}
// Dependencies: {}
# [doc = " This is the entrypoint for a hot plugged rustc_codegen_cranelift"] # [no_mangle] pub fn __rustc_codegen_backend () -> Box < dyn CodegenBackend > { Box :: new (CraneliftCodegenBackend { config : None }) }
};
}
