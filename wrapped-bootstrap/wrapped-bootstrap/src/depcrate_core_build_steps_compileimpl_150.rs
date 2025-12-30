// Generated macro for impl_150 (impl)
macro_rules! Depcrate_core_build_steps_compileimpl_150 {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"impl_150"}
// Dependencies: {}
impl StdLink { pub fn from_std (std : Std , host_compiler : Compiler) -> Self { Self { compiler : host_compiler , target_compiler : std . build_compiler , target : std . target , crates : std . crates , force_recompile : std . force_recompile , } } }
};
}
