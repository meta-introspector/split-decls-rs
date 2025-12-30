// Generated macro for create_dir (function)
macro_rules! Depcrate_utilcreate_dir {
() => {
// Module: crate::util
// Provides: {"create_dir"}
// Dependencies: {}
# [doc = " Wraps `fs::create_dir` with a nicer error message."] pub fn create_dir < P : AsRef < Path > > (path : P) -> Result < () > { fs :: create_dir (& path) . with_context (| | format ! ("failed to create dir '{}'" , path . as_ref () . display ())) ? ; Ok (()) }
};
}
