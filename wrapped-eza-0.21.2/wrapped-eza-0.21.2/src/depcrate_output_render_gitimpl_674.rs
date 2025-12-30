// Generated macro for impl_674 (impl)
macro_rules! Depcrate_output_render_gitimpl_674 {
() => {
// Module: crate::output::render::git
// Provides: {"impl_674"}
// Dependencies: {}
impl f :: SubdirGitRepo { pub fn render (self , colours : & dyn RepoColours) -> TextCell { let branch_name = match self . branch { Some (name) => match name . as_ref () { "main" | "master" => colours . branch_main () . paint (name) , _ => colours . branch_other () . paint (name) , } , None => colours . no_repo () . paint ("-") , } ; if let Some (status) = self . status { TextCell { width : DisplayWidth :: from (2) + DisplayWidth :: from (branch_name . as_str ()) , contents : vec ! [status . render (colours) , Style :: default () . paint (" ") , branch_name ,] . into () , } } else { TextCell { width : DisplayWidth :: from (branch_name . as_str ()) , contents : vec ! [branch_name] . into () , } } } }
};
}
