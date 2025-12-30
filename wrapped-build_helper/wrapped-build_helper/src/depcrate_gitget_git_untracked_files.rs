// Generated macro for get_git_untracked_files (function)
macro_rules! Depcrate_gitget_git_untracked_files {
() => {
// Module: crate::git
// Provides: {"get_git_untracked_files"}
// Dependencies: {}
# [doc = " Returns the files that haven't been added to git yet."] pub fn get_git_untracked_files (git_dir : Option < & Path >) -> Result < Option < Vec < String > > , String > { let mut git = Command :: new ("git") ; if let Some (git_dir) = git_dir { git . current_dir (git_dir) ; } let files = output_result (git . arg ("ls-files") . arg ("--others") . arg ("--exclude-standard")) ? . lines () . map (| s | s . trim () . to_owned ()) . collect () ; Ok (Some (files)) }
};
}
