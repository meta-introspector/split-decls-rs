// Generated macro for set_file_handle_times (function)
macro_rules! Depcrateset_file_handle_times {
() => {
// Module: crate
// Provides: {"set_file_handle_times"}
// Dependencies: {}
# [doc = " Set the last access and modification times for a file handle."] # [doc = ""] # [doc = " This function will either or both of  the `atime` and `mtime` metadata"] # [doc = " fields for a file handle , returning any error encountered. If `None` is"] # [doc = " specified then the time won't be updated. If `None` is specified for both"] # [doc = " options then no action is taken."] pub fn set_file_handle_times (f : & fs :: File , atime : Option < FileTime > , mtime : Option < FileTime > ,) -> io :: Result < () > { imp :: set_file_handle_times (f , atime , mtime) }
};
}
