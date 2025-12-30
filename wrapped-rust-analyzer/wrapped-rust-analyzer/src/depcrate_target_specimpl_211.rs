// Generated macro for impl_211 (impl)
macro_rules! Depcrate_target_specimpl_211 {
() => {
// Module: crate::target_spec
// Provides: {"impl_211"}
// Dependencies: {}
impl ProjectJsonTargetSpec { pub (crate) fn runnable_args (& self , kind : & RunnableKind) -> Option < Runnable > { match kind { RunnableKind :: Bin => { for runnable in & self . shell_runnables { if matches ! (runnable . kind , project_model :: project_json :: RunnableKind :: Run) { return Some (runnable . clone ()) ; } } None } RunnableKind :: Test { test_id , .. } => { for runnable in & self . shell_runnables { if matches ! (runnable . kind , project_model :: project_json :: RunnableKind :: TestOne) { let mut runnable = runnable . clone () ; let replaced_args : Vec < _ > = runnable . args . iter () . map (| arg | arg . replace ("{test_id}" , & test_id . to_string ())) . map (| arg | arg . replace ("{label}" , & self . label)) . collect () ; runnable . args = replaced_args ; return Some (runnable) ; } } None } RunnableKind :: TestMod { .. } => None , RunnableKind :: Bench { .. } => None , RunnableKind :: DocTest { .. } => None , } } }
};
}
