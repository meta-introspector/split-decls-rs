// Generated macro for create_new_executable (function)
macro_rules! Depcrate_utilcreate_new_executable {
() => {
// Module: crate::util
// Provides: {"create_new_executable"}
// Dependencies: {}
# [doc = " Wraps `fs::OpenOptions::create_new().open()` as executable, with a nicer error message."] pub fn create_new_executable < P : AsRef < Path > > (path : P) -> Result < fs :: File > { let mut options = fs :: OpenOptions :: new () ; options . write (true) . create_new (true) ; # [cfg (unix)] options . mode (0o755) ; let file = options . open (& path) . with_context (| | format ! ("failed to create file '{}'" , path . as_ref () . display ())) ? ; Ok (file) }
};
}
