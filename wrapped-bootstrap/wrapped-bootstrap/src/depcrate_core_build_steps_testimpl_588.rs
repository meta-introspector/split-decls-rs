// Generated macro for impl_588 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_588 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_588"}
// Dependencies: {}
impl Step for HtmlCheck { type Output = () ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let builder = run . builder ; let run = run . path ("src/tools/html-checker") ; run . lazy_default_condition (Box :: new (| | check_if_tidy_is_installed (builder))) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (HtmlCheck { target : run . target }) ; } fn run (self , builder : & Builder < '_ >) { if ! check_if_tidy_is_installed (builder) { eprintln ! ("not running HTML-check tool because `tidy` is missing") ; eprintln ! ("You need the HTML tidy tool https://www.html-tidy.org/, this tool is *not* part of the rust project and needs to be installed separately, for example via your package manager.") ; panic ! ("Cannot run html-check tests") ; } builder . run_default_doc_steps () ; builder . ensure (crate :: core :: build_steps :: doc :: Rustc :: for_stage (builder , builder . top_stage , self . target ,)) ; builder . tool_cmd (Tool :: HtmlChecker) . delay_failure () . arg (builder . doc_out (self . target)) . run (builder) ; } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: test ("html-check" , self . target)) } }
};
}
