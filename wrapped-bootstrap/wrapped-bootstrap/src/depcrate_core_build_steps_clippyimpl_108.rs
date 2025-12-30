// Generated macro for impl_108 (impl)
macro_rules! Depcrate_core_build_steps_clippyimpl_108 {
() => {
// Module: crate::core::build_steps::clippy
// Provides: {"impl_108"}
// Dependencies: {}
impl Rustc { fn new (builder : & Builder < '_ > , target : TargetSelection , config : LintConfig , crates : Vec < String > ,) -> Self { Self { build_compiler : prepare_compiler_for_check (builder , target , Mode :: Rustc) , target , config , crates , } } }
};
}
