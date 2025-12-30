// Generated macro for open (function)
macro_rules! Depcrate_fsopen {
() => {
// Module: crate::fs
// Provides: {"open"}
// Dependencies: {}
pub fn open (name : & str , flags : OpenOption , mode : AccessPermission) -> io :: Result < FileDescriptor > { let mask = * UMASK . lock () ; with_relative_filename (name , | name | { debug ! ("Open {name}, {flags:?}, {mode:?}") ; let fs = FILESYSTEM . get () . ok_or (Errno :: Inval) ? ; let file = fs . open (name , flags , mode . bitand (mask)) ? ; let fd = insert_object (file) ? ; Ok (fd) }) }
};
}
