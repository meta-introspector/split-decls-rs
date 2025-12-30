// Generated macro for last_modified_from_file (function)
macro_rules! Depcrate_util_fslast_modified_from_file {
() => {
// Module: crate::util::fs
// Provides: {"last_modified_from_file"}
// Dependencies: {}
# [doc = " Returns the last modified time for the given file as a Jiff timestamp."] # [doc = ""] # [doc = " If there was a problem accessing the last modified time or if it could not"] # [doc = " fit in a Jiff timestamp, then a warning message is logged and `None` is"] # [doc = " returned."] # [doc = ""] # [doc = " The path given should be the path to the given file. It is used for"] # [doc = " diagnostic purposes."] pub (crate) fn last_modified_from_file (_path : & Path , file : & File ,) -> Option < Timestamp > { let md = match file . metadata () { Ok (md) => md , Err (_err) => { warn ! ("failed to get metadata (for last modified time) \
                 for {}: {_err}" , _path . display () ,) ; return None ; } } ; let systime = match md . modified () { Ok (systime) => systime , Err (_err) => { warn ! ("failed to get last modified time for {}: {_err}" , _path . display ()) ; return None ; } } ; let timestamp = match Timestamp :: try_from (systime) { Ok (timestamp) => timestamp , Err (_err) => { warn ! ("system time {systime:?} out of bounds \
                 for Jiff timestamp for last modified time \
                 from {}: {_err}" , _path . display () ,) ; return None ; } } ; Some (timestamp) }
};
}
