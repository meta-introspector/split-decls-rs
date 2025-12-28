macro_rules! set_file_time_no_err {
    () => {
        # [doc = " Changes the filesystem mtime (and atime if possible) for the given file."] # [doc = ""] # [doc = " This intentionally does not return an error, as this is sometimes not"] # [doc = " supported on network filesystems. For the current uses in Cargo, this is a"] # [doc = " \"best effort\" approach, and errors shouldn't be propagated."] pub fn set_file_time_no_err < P : AsRef < Path > > (path : P , time : FileTime) { let path = path . as_ref () ; match filetime :: set_file_times (path , time , time) { Ok (()) => tracing :: debug ! ("set file mtime {} to {}" , path . display () , time) , Err (e) => tracing :: warn ! ("could not set mtime of {} to {}: {:?}" , path . display () , time , e) , } }
    };
}

set_file_time_no_err!();