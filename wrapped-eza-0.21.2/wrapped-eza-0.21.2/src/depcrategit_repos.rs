// Generated macro for git_repos (function)
macro_rules! Depcrategit_repos {
() => {
// Module: crate
// Provides: {"git_repos"}
// Dependencies: {}
# [cfg (feature = "git")] fn git_repos (options : & Options , args : & [& OsStr]) -> bool { let option_enabled = match options . view . mode { Mode :: Details (details :: Options { table : Some (ref table) , .. }) | Mode :: GridDetails (grid_details :: Options { details : details :: Options { table : Some (ref table) , .. } , .. }) => table . columns . subdir_git_repos || table . columns . subdir_git_repos_no_stat , _ => false , } ; if option_enabled { let paths : Vec < PathBuf > = args . iter () . map (PathBuf :: from) . collect :: < Vec < PathBuf > > () ; let mut files : Vec < PathBuf > = Vec :: new () ; for path in paths { get_files_in_dir (& mut files , path) ; } let repos : Vec < bool > = files . iter () . map (git2 :: Repository :: open) . map (| repo | repo . is_ok ()) . collect () ; repos . contains (& true) } else { false } }
};
}
