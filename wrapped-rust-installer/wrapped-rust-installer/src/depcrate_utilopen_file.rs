// Generated macro for open_file (function)
macro_rules! Depcrate_utilopen_file {
() => {
// Module: crate::util
// Provides: {"open_file"}
// Dependencies: {}
# [doc = " Wraps `fs::File::open()` with a nicer error message."] pub fn open_file < P : AsRef < Path > > (path : P) -> Result < fs :: File > { let file = fs :: File :: open (& path) . with_context (| | format ! ("failed to open file '{}'" , path . as_ref () . display ())) ? ; Ok (file) }
};
}
