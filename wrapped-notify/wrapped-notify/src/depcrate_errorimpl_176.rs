// Generated macro for impl_176 (impl)
macro_rules! Depcrate_errorimpl_176 {
() => {
// Module: crate::error
// Provides: {"impl_176"}
// Dependencies: {}
impl Error { # [doc = " Adds a path to the error."] pub fn add_path (mut self , path : PathBuf) -> Self { self . paths . push (path) ; self } # [doc = " Replaces the paths for the error."] pub fn set_paths (mut self , paths : Vec < PathBuf >) -> Self { self . paths = paths ; self } # [doc = " Creates a new Error with empty paths given its kind."] pub fn new (kind : ErrorKind) -> Self { Self { kind , paths : Vec :: new () , } } # [doc = " Creates a new generic Error from a message."] pub fn generic (msg : & str) -> Self { Self :: new (ErrorKind :: Generic (msg . into ())) } # [doc = " Creates a new i/o Error from a stdlib `io::Error`."] pub fn io (err : io :: Error) -> Self { Self :: new (ErrorKind :: Io (err)) } # [doc = " Similar to [`Error::io`], but specifically handles [`io::ErrorKind::NotFound`]."] pub fn io_watch (err : io :: Error) -> Self { if err . kind () == io :: ErrorKind :: NotFound { Self :: path_not_found () } else { Self :: io (err) } } # [doc = " Creates a new \"path not found\" error."] pub fn path_not_found () -> Self { Self :: new (ErrorKind :: PathNotFound) } # [doc = " Creates a new \"watch not found\" error."] pub fn watch_not_found () -> Self { Self :: new (ErrorKind :: WatchNotFound) } # [doc = " Creates a new \"invalid config\" error from the given `Config`."] pub fn invalid_config (config : & Config) -> Self { Self :: new (ErrorKind :: InvalidConfig (* config)) } }
};
}
