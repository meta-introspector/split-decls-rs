// Generated macro for unlink (function)
macro_rules! Depcrate_fsunlink {
() => {
// Module: crate::fs
// Provides: {"unlink"}
// Dependencies: {}
pub fn unlink (path : & str) -> io :: Result < () > { with_relative_filename (path , | path | { FILESYSTEM . get () . ok_or (Errno :: Inval) ? . unlink (path) }) }
};
}
