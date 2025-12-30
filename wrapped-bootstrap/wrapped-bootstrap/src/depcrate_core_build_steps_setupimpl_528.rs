// Generated macro for impl_528 (impl)
macro_rules! Depcrate_core_build_steps_setupimpl_528 {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"impl_528"}
// Dependencies: {}
impl Step for Profile { type Output = () ; const DEFAULT : bool = true ; fn should_run (mut run : ShouldRun < '_ >) -> ShouldRun < '_ > { for choice in Profile :: all () { run = run . alias (choice . as_str ()) ; } run } fn make_run (run : RunConfig < '_ >) { if run . builder . config . dry_run () { return ; } let path = & run . builder . config . config . clone () . unwrap_or (PathBuf :: from ("bootstrap.toml")) ; if path . exists () { eprintln ! () ; eprintln ! ("ERROR: you asked for a new config file, but one already exists at `{}`" , t ! (path . canonicalize ()) . display ()) ; match prompt_user ("Do you wish to override the existing configuration (which will allow the setup process to continue)?: [y/N]" ,) { Ok (Some (PromptResult :: Yes)) => { t ! (fs :: remove_file (path)) ; } _ => { println ! ("Exiting.") ; crate :: exit ! (1) ; } } } let profile = if run . paths . len () > 1 { t ! (interactive_path ()) } else { run . paths . first () . unwrap () . assert_single_path () . path . as_path () . as_os_str () . to_str () . unwrap () . parse () . unwrap () } ; run . builder . ensure (profile) ; } fn run (self , builder : & Builder < '_ >) { setup (& builder . build . config , self) ; } }
};
}
