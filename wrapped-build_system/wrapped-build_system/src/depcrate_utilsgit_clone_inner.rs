// Generated macro for git_clone_inner (function)
macro_rules! Depcrate_utilsgit_clone_inner {
() => {
// Module: crate::utils
// Provides: {"git_clone_inner"}
// Dependencies: {}
fn git_clone_inner (to_clone : & str , dest : & Path , shallow_clone : bool , repo_name : String ,) -> Result < CloneResult , String > { if dest . is_dir () { return Ok (CloneResult { ran_clone : false , repo_name , repo_dir : dest . display () . to_string () , }) ; } let mut command : Vec < & dyn AsRef < OsStr > > = vec ! [& "git" , & "clone" , & to_clone , & dest] ; if shallow_clone { command . push (& "--depth") ; command . push (& "1") ; } run_command_with_output (& command , None) ? ; Ok (CloneResult { ran_clone : true , repo_name , repo_dir : dest . display () . to_string () }) }
};
}
