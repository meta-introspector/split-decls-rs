// Generated macro for run (function)
macro_rules! Depcrate_clone_gccrun {
() => {
// Module: crate::clone_gcc
// Provides: {"run"}
// Dependencies: {}
pub fn run () -> Result < () , String > { let Some (args) = Args :: new () ? else { return Ok (()) ; } ; let result = git_clone ("https://github.com/rust-lang/gcc" , Some (& args . out_path) , false) ? ; if result . ran_clone { let gcc_commit = args . config_info . get_gcc_commit () ? ; println ! ("Checking out GCC commit `{gcc_commit}`...") ; run_command_with_output (& [& "git" , & "checkout" , & gcc_commit] , Some (Path :: new (& result . repo_dir)) ,) ? ; } else { println ! ("There is already a GCC folder in `{}`, leaving things as is..." , args . out_path . display ()) ; } Ok (()) }
};
}
