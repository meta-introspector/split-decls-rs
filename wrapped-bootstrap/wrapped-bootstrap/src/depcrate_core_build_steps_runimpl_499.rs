// Generated macro for impl_499 (impl)
macro_rules! Depcrate_core_build_steps_runimpl_499 {
() => {
// Module: crate::core::build_steps::run
// Provides: {"impl_499"}
// Dependencies: {}
impl Step for UnicodeTableGenerator { type Output = () ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/unicode-table-generator") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (UnicodeTableGenerator) ; } fn run (self , builder : & Builder < '_ >) { let mut cmd = builder . tool_cmd (Tool :: UnicodeTableGenerator) ; cmd . arg (builder . src . join ("library/core/src/unicode/unicode_data.rs")) ; cmd . run (builder) ; } }
};
}
