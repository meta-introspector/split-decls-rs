// Generated macro for create_new_file (function)
macro_rules! Depcrate_utilcreate_new_file {
() => {
// Module: crate::util
// Provides: {"create_new_file"}
// Dependencies: {}
# [doc = " Wraps `fs::OpenOptions::create_new().open()`, with a nicer error message."] pub fn create_new_file < P : AsRef < Path > > (path : P) -> Result < fs :: File > { let file = fs :: OpenOptions :: new () . write (true) . create_new (true) . open (& path) . with_context (| | format ! ("failed to create file '{}'" , path . as_ref () . display ())) ? ; Ok (file) }
};
}
