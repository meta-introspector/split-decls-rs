// Generated macro for impl_47 (impl)
macro_rules! Depcrate_core_build_steps_checkimpl_47 {
() => {
// Module: crate::core::build_steps::check
// Provides: {"impl_47"}
// Dependencies: {}
impl CompilerForCheck { pub fn build_compiler (& self) -> Compiler { self . build_compiler } # [doc = " If there are any rustc rmeta artifacts available, configure the Cargo invocation"] # [doc = " so that the artifact being built can find them."] pub fn configure_cargo (& self , cargo : & mut Cargo) { if let Some (sysroot) = & self . rustc_rmeta_sysroot { sysroot . configure_cargo (cargo) ; } if let Some (sysroot) = & self . std_rmeta_sysroot { sysroot . configure_cargo (cargo) ; } } }
};
}
