// Generated macro for run_in_tmpdir (function)
macro_rules! Depcrate_scoped_runrun_in_tmpdir {
() => {
// Module: crate::scoped_run
// Provides: {"run_in_tmpdir"}
// Dependencies: {}
# [doc = " This function is designed for running commands in a temporary directory that is cleared after"] # [doc = " the function ends."] # [doc = ""] # [doc = " What this function does:"] # [doc = " 1. Creates a temporary directory (`tmpdir`)"] # [doc = " 2. Copies all files from the current directory to `tmpdir`"] # [doc = " 3. Changes the current working directory to `tmpdir`"] # [doc = " 4. Calls `callback`"] # [doc = " 5. Switches working directory back to the original one"] # [doc = " 6. Removes `tmpdir`"] pub fn run_in_tmpdir < F : FnOnce () > (callback : F) { let original_dir = cwd () ; let tmpdir = original_dir . join ("../temporary-directory") ; fs :: copy_dir_all ("." , & tmpdir) ; std :: env :: set_current_dir (& tmpdir) . unwrap () ; callback () ; std :: env :: set_current_dir (original_dir) . unwrap () ; fs :: remove_dir_all (tmpdir) ; }
};
}
