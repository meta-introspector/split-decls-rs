// Generated macro for DirTable (struct)
macro_rules! Depcrate_shims_unix_fsDirTable {
() => {
// Module: crate::shims::unix::fs
// Provides: {"DirTable"}
// Dependencies: {}
# [doc = " The table of open directories."] # [doc = " Curiously, Unix/POSIX does not unify this into the \"file descriptor\" concept... everything"] # [doc = " is a file, except a directory is not?"] # [derive (Debug)] pub struct DirTable { # [doc = " Directory iterators used to emulate libc \"directory streams\", as used in opendir, readdir,"] # [doc = " and closedir."] # [doc = ""] # [doc = " When opendir is called, a directory iterator is created on the host for the target"] # [doc = " directory, and an entry is stored in this hash map, indexed by an ID which represents"] # [doc = " the directory stream. When readdir is called, the directory stream ID is used to look up"] # [doc = " the corresponding ReadDir iterator from this map, and information from the next"] # [doc = " directory entry is returned. When closedir is called, the ReadDir iterator is removed from"] # [doc = " the map."] streams : FxHashMap < u64 , OpenDir > , # [doc = " ID number to be used by the next call to opendir"] next_id : u64 , }
};
}
