// Generated macro for create_dir_all (function)
macro_rules! Depcrate_utilcreate_dir_all {
() => {
// Module: crate::util
// Provides: {"create_dir_all"}
// Dependencies: {}
# [doc = " Wraps `fs::create_dir_all` with a nicer error message."] pub fn create_dir_all < P : AsRef < Path > > (path : P) -> Result < () > { fs :: create_dir_all (& path) . with_context (| | format ! ("failed to create dir '{}'" , path . as_ref () . display ())) ? ; Ok (()) }
};
}
