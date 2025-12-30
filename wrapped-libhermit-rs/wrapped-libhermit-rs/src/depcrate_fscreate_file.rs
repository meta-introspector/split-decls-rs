// Generated macro for create_file (function)
macro_rules! Depcrate_fscreate_file {
() => {
// Module: crate::fs
// Provides: {"create_file"}
// Dependencies: {}
pub fn create_file (name : & str , data : & 'static [u8] , mode : AccessPermission) -> io :: Result < () > { with_relative_filename (name , | name | { FILESYSTEM . get () . ok_or (Errno :: Inval) ? . create_file (name , data , mode) }) }
};
}
