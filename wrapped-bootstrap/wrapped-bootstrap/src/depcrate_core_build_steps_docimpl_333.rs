// Generated macro for impl_333 (impl)
macro_rules! Depcrate_core_build_steps_docimpl_333 {
() => {
// Module: crate::core::build_steps::doc
// Provides: {"impl_333"}
// Dependencies: {}
impl Step for UnstableBookGen { type Output = () ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let builder = run . builder ; run . path ("src/tools/unstable-book-gen") . default_condition (builder . config . docs) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (UnstableBookGen { target : run . target }) ; } fn run (self , builder : & Builder < '_ >) { let target = self . target ; builder . info (& format ! ("Generating unstable book md files ({target})")) ; let out = builder . md_doc_out (target) . join ("unstable-book") ; builder . create_dir (& out) ; builder . remove_dir (& out) ; let mut cmd = builder . tool_cmd (Tool :: UnstableBookGen) ; cmd . arg (builder . src . join ("library")) ; cmd . arg (builder . src . join ("compiler")) ; cmd . arg (builder . src . join ("src")) ; cmd . arg (out) ; cmd . run (builder) ; } }
};
}
