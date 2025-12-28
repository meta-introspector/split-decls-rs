macro_rules! join_paths {
    () => {
        # [doc = " Joins paths into a string suitable for the `PATH` environment variable."] # [doc = ""] # [doc = " This is equivalent to [`std::env::join_paths`], but includes a more"] # [doc = " detailed error message. The given `env` argument is the name of the"] # [doc = " environment variable this is will be used for, which is included in the"] # [doc = " error message."] pub fn join_paths < T : AsRef < OsStr > > (paths : & [T] , env : & str) -> Result < OsString > { env :: join_paths (paths . iter ()) . with_context (| | { let mut message = format ! ("failed to join paths from `${env}` together\n\n\
             Check if any of path segments listed below contain an \
             unterminated quote character or path separator:") ; for path in paths { use std :: fmt :: Write ; write ! (& mut message , "\n    {:?}" , Path :: new (path)) . unwrap () ; } message }) }
    };
}

join_paths!();