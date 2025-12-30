// Generated macro for impl_598 (impl)
macro_rules! Depcrate_unwind_moduleimpl_598 {
() => {
// Module: crate::unwind_module
// Provides: {"impl_598"}
// Dependencies: {}
# [cfg (feature = "jit")] impl UnwindModule < cranelift_jit :: JITModule > { pub (crate) fn finalize_definitions (mut self) -> cranelift_jit :: JITModule { self . module . finalize_definitions () . unwrap () ; unsafe { self . unwind_context . register_jit (& self . module) } ; self . module } }
};
}
