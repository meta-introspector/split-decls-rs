// Generated macro for install_git_hook_maybe (function)
macro_rules! Depcrate_core_build_steps_setupinstall_git_hook_maybe {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"install_git_hook_maybe"}
// Dependencies: {}
fn install_git_hook_maybe (builder : & Builder < '_ > , config : & Config) -> io :: Result < () > { let git = helpers :: git (Some (& config . src)) . args (["rev-parse" , "--git-common-dir"]) . run_capture (builder) . stdout () ; let git = PathBuf :: from (git . trim ()) ; let hooks_dir = git . join ("hooks") ; let dst = hooks_dir . join ("pre-push") ; if dst . exists () { return Ok (()) ; } println ! ("\nRust's CI will automatically fail if it doesn't pass `tidy`, the internal tool for ensuring code quality.
If you'd like, x.py can install a git hook for you that will automatically run `test tidy` before
pushing your code to ensure your code is up to par. If you decide later that this behavior is
undesirable, simply delete the `pre-push` file from .git/hooks.") ; if prompt_user ("Would you like to install the git hook?: [y/N]") ? != Some (PromptResult :: Yes) { println ! ("Ok, skipping installation!") ; return Ok (()) ; } if ! hooks_dir . exists () { let _ = fs :: create_dir (hooks_dir) ; } let src = config . src . join ("src") . join ("etc") . join ("pre-push.sh") ; match fs :: hard_link (src , & dst) { Err (e) => { eprintln ! ("ERROR: could not create hook {}: do you already have the git hook installed?\n{}" , dst . display () , e) ; return Err (e) ; } Ok (_) => println ! ("Linked `src/etc/pre-push.sh` to `.git/hooks/pre-push`") , } ; Ok (()) }
};
}
