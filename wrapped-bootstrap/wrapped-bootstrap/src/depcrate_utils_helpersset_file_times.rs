// Generated macro for set_file_times (function)
macro_rules! Depcrate_utils_helpersset_file_times {
() => {
// Module: crate::utils::helpers
// Provides: {"set_file_times"}
// Dependencies: {}
# [doc = " Sets the file times for a given file at `path`."] pub fn set_file_times < P : AsRef < Path > > (path : P , times : fs :: FileTimes) -> io :: Result < () > { let f = if cfg ! (windows) { fs :: File :: options () . write (true) . open (path) ? } else { fs :: File :: open (path) ? } ; f . set_times (times) }
};
}
