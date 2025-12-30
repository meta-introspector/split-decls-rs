// Generated macro for impl_497 (impl)
macro_rules! Depcrate_core_build_steps_runimpl_497 {
() => {
// Module: crate::core::build_steps::run
// Provides: {"impl_497"}
// Dependencies: {}
impl Step for GenerateCompletions { type Output = () ; # [doc = " Uses `clap_complete` to generate shell completions."] fn run (self , builder : & Builder < '_ >) { for (shell , path) in get_completion_paths (builder) { if let Some (comp) = get_completion (shell , & path) { std :: fs :: write (& path , comp) . unwrap_or_else (| e | { panic ! ("writing completion into {} failed: {e:?}" , path . display ()) }) ; } } } fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("generate-completions") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (GenerateCompletions) ; } }
};
}
