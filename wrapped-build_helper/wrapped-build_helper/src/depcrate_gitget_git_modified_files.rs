// Generated macro for get_git_modified_files (function)
macro_rules! Depcrate_gitget_git_modified_files {
() => {
// Module: crate::git
// Provides: {"get_git_modified_files"}
// Dependencies: {}
# [doc = " Returns the files that have been modified in the current branch compared to the master branch."] # [doc = " This includes committed changes, uncommitted changes, and changes that are not even staged."] # [doc = ""] # [doc = " The `extensions` parameter can be used to filter the files by their extension."] # [doc = " Does not include removed files."] # [doc = " If `extensions` is empty, all files will be returned."] pub fn get_git_modified_files (config : & GitConfig < '_ > , git_dir : Option < & Path > , extensions : & [& str] ,) -> Result < Vec < String > , String > { let Some (merge_base) = get_closest_upstream_commit (git_dir , config , CiEnv :: None) ? else { return Err ("No upstream commit was found" . to_string ()) ; } ; let mut git = Command :: new ("git") ; if let Some (git_dir) = git_dir { git . current_dir (git_dir) ; } let files = output_result (git . args (["diff-index" , "--name-status" , merge_base . trim ()])) ? . lines () . filter_map (| f | { let (status , name) = f . trim () . split_once (char :: is_whitespace) . unwrap () ; if status == "D" { None } else if Path :: new (name) . extension () . map_or (extensions . is_empty () , | ext | { extensions . is_empty () || extensions . contains (& ext . to_str () . unwrap ()) }) { Some (name . to_owned ()) } else { None } }) . collect () ; Ok (files) }
};
}
