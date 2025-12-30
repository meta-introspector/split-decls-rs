// Generated macro for read_stat (function)
macro_rules! Depcrate_fsread_stat {
() => {
// Module: crate::fs
// Provides: {"read_stat"}
// Dependencies: {}
pub fn read_stat (name : & str) -> io :: Result < FileAttr > { with_relative_filename (name , | name | { FILESYSTEM . get () . ok_or (Errno :: Inval) ? . stat (name) }) }
};
}
