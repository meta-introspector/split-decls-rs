// Generated macro for prepare_rand (function)
macro_rules! Depcrate_prepareprepare_rand {
() => {
// Module: crate::prepare
// Provides: {"prepare_rand"}
// Dependencies: {}
fn prepare_rand () -> Result < () , String > { let file_path = "patches/crates/0001-Remove-deny-warnings.patch" ; let rand_dir = Path :: new ("build/rand") ; println ! ("[GIT] apply `{file_path}`") ; let path = Path :: new ("../..") . join (file_path) ; run_command_with_output (& [& "git" , & "apply" , & path] , Some (rand_dir)) ? ; run_command_with_output (& [& "git" , & "add" , & "-A"] , Some (rand_dir)) ? ; run_command_with_output (& [& "git" , & "commit" , & "--no-gpg-sign" , & "-m" , & format ! ("Patch {}" , path . display ())] , Some (rand_dir) ,) ? ; Ok (()) }
};
}
