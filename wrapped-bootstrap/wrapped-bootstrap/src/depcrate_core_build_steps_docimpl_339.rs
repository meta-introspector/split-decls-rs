// Generated macro for impl_339 (impl)
macro_rules! Depcrate_core_build_steps_docimpl_339 {
() => {
// Module: crate::core::build_steps::doc
// Provides: {"impl_339"}
// Dependencies: {}
impl Step for Reference { type Output = () ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let builder = run . builder ; run . path ("src/doc/reference") . default_condition (builder . config . docs) } fn make_run (run : RunConfig < '_ >) { let stage = if run . builder . config . is_explicit_stage () || run . builder . top_stage >= 2 { run . builder . top_stage } else { 2 } ; run . builder . ensure (Reference { build_compiler : prepare_doc_compiler (run . builder , run . target , stage) , target : run . target , }) ; } # [doc = " Builds the reference book."] fn run (self , builder : & Builder < '_ >) { builder . require_submodule ("src/doc/reference" , None) ; builder . std (self . build_compiler , builder . config . host_target) ; builder . ensure (RustbookSrc { target : self . target , name : "reference" . to_owned () , src : builder . src . join ("src/doc/reference") , build_compiler : Some (self . build_compiler) , parent : Some (self) , languages : vec ! [] , }) ; } }
};
}
