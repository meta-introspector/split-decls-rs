// Generated macro for impl_295 (impl)
macro_rules! Depcrate_core_build_steps_docimpl_295 {
() => {
// Module: crate::core::build_steps::doc
// Provides: {"impl_295"}
// Dependencies: {}
impl Step for UnstableBook { type Output = () ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let builder = run . builder ; run . path ("src/doc/unstable-book") . default_condition (builder . config . docs) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (UnstableBook { target : run . target }) ; } fn run (self , builder : & Builder < '_ >) { builder . ensure (UnstableBookGen { target : self . target }) ; builder . ensure (RustbookSrc { target : self . target , name : "unstable-book" . to_owned () , src : builder . md_doc_out (self . target) . join ("unstable-book") , parent : Some (self) , languages : vec ! [] , build_compiler : None , }) } }
};
}
