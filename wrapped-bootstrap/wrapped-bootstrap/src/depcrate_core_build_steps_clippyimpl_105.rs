// Generated macro for impl_105 (impl)
macro_rules! Depcrate_core_build_steps_clippyimpl_105 {
() => {
// Module: crate::core::build_steps::clippy
// Provides: {"impl_105"}
// Dependencies: {}
impl Std { fn new (builder : & Builder < '_ > , target : TargetSelection , config : LintConfig , crates : Vec < String > ,) -> Self { Self { build_compiler : builder . compiler (builder . top_stage , builder . host_target) , target , config , crates , } } fn from_build_compiler (build_compiler : Compiler , target : TargetSelection , config : LintConfig , crates : Vec < String > ,) -> Self { Self { build_compiler , target , config , crates } } }
};
}
