// Generated macro for impl_548 (impl)
macro_rules! Depcrate_core_build_steps_setupimpl_548 {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"impl_548"}
// Dependencies: {}
impl Step for Editor { type Output = () ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("editor") } fn make_run (run : RunConfig < '_ >) { if run . builder . config . dry_run () { return ; } if let [cmd] = & run . paths [..] && cmd . assert_single_path () . path . as_path () . as_os_str () == "editor" { run . builder . ensure (Editor) ; } } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let config = & builder . config ; if config . dry_run () { return ; } match EditorKind :: prompt_user () { Ok (editor_kind) => { if let Some (editor_kind) = editor_kind { while ! t ! (create_editor_settings_maybe (config , & editor_kind)) { } } else { println ! ("Ok, skipping editor setup!") ; } } Err (e) => eprintln ! ("Could not determine the editor: {e}") , } } }
};
}
