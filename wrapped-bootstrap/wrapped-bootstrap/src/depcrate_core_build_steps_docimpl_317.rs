// Generated macro for impl_317 (impl)
macro_rules! Depcrate_core_build_steps_docimpl_317 {
() => {
// Module: crate::core::build_steps::doc
// Provides: {"impl_317"}
// Dependencies: {}
impl Rustc { # [doc = " Document `stage` compiler for the given `target`."] pub (crate) fn for_stage (builder : & Builder < '_ > , stage : u32 , target : TargetSelection) -> Self { let build_compiler = prepare_doc_compiler (builder , target , stage) ; Self :: from_build_compiler (builder , build_compiler , target) } fn from_build_compiler (builder : & Builder < '_ > , build_compiler : Compiler , target : TargetSelection ,) -> Self { let crates = builder . in_tree_crates ("rustc-main" , Some (target)) . into_iter () . map (| krate | krate . name . to_string ()) . collect () ; Self { build_compiler , target , crates } } }
};
}
