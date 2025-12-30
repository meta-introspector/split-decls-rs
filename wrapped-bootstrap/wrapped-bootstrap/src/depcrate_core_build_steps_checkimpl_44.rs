// Generated macro for impl_44 (impl)
macro_rules! Depcrate_core_build_steps_checkimpl_44 {
() => {
// Module: crate::core::build_steps::check
// Provides: {"impl_44"}
// Dependencies: {}
impl Rustc { pub fn new (builder : & Builder < '_ > , target : TargetSelection , crates : Vec < String >) -> Self { let build_compiler = prepare_compiler_for_check (builder , target , Mode :: Rustc) ; Self :: from_build_compiler (build_compiler , target , crates) } fn from_build_compiler (build_compiler : CompilerForCheck , target : TargetSelection , crates : Vec < String > ,) -> Self { Self { build_compiler , target , crates } } }
};
}
