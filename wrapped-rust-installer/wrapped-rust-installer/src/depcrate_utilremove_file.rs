// Generated macro for remove_file (function)
macro_rules! Depcrate_utilremove_file {
() => {
// Module: crate::util
// Provides: {"remove_file"}
// Dependencies: {}
# [doc = " Wrap `fs::remove_file` with a nicer error message"] pub fn remove_file < P : AsRef < Path > > (path : P) -> Result < () > { fs :: remove_file (path . as_ref ()) . with_context (| | format ! ("failed to remove file '{}'" , path . as_ref () . display ())) ? ; Ok (()) }
};
}
