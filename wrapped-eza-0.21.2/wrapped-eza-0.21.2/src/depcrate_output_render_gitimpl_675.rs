// Generated macro for impl_675 (impl)
macro_rules! Depcrate_output_render_gitimpl_675 {
() => {
// Module: crate::output::render::git
// Provides: {"impl_675"}
// Dependencies: {}
impl f :: SubdirGitRepoStatus { pub fn render (self , colours : & dyn RepoColours) -> ANSIString < 'static > { match self { Self :: NoRepo => colours . no_repo () . paint ("-") , Self :: GitClean => colours . git_clean () . paint ("|") , Self :: GitDirty => colours . git_dirty () . paint ("+") , } } }
};
}
