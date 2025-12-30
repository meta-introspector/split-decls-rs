// Generated macro for clone_and_setup (function)
macro_rules! Depcrate_prepareclone_and_setup {
() => {
// Module: crate::prepare
// Provides: {"clone_and_setup"}
// Dependencies: {}
fn clone_and_setup < F > (repo_url : & str , checkout_commit : & str , extra : Option < F >) -> Result < () , String > where F : Fn (& Path) -> Result < () , String > , { let clone_result = git_clone_root_dir (repo_url , Path :: new (crate :: BUILD_DIR) , false) ? ; if ! clone_result . ran_clone { println ! ("`{}` has already been cloned" , clone_result . repo_name) ; } let repo_path = Path :: new (crate :: BUILD_DIR) . join (& clone_result . repo_name) ; run_command (& [& "git" , & "checkout" , & "--" , & "."] , Some (& repo_path)) ? ; run_command (& [& "git" , & "checkout" , & checkout_commit] , Some (& repo_path)) ? ; if let Some (extra) = extra { extra (& repo_path) ? ; } Ok (()) }
};
}
