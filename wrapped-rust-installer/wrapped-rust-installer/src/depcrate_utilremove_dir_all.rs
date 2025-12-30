// Generated macro for remove_dir_all (function)
macro_rules! Depcrate_utilremove_dir_all {
() => {
// Module: crate::util
// Provides: {"remove_dir_all"}
// Dependencies: {}
# [doc = " Wraps `remove_dir_all` with a nicer error message."] pub fn remove_dir_all < P : AsRef < Path > > (path : P) -> Result < () > { fs :: remove_dir_all (path . as_ref ()) . with_context (| | format ! ("failed to remove dir '{}'" , path . as_ref () . display ())) ? ; Ok (()) }
};
}
