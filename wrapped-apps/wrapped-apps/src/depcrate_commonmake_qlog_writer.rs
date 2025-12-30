// Generated macro for make_qlog_writer (function)
macro_rules! Depcrate_commonmake_qlog_writer {
() => {
// Module: crate::common
// Provides: {"make_qlog_writer"}
// Dependencies: {}
# [doc = " Makes a buffered writer for a qlog."] pub fn make_qlog_writer (dir : & std :: ffi :: OsStr , role : & str , id : & str ,) -> std :: io :: BufWriter < std :: fs :: File > { let mut path = std :: path :: PathBuf :: from (dir) ; let filename = format ! ("{role}-{id}.sqlog") ; path . push (filename) ; match std :: fs :: File :: create (& path) { Ok (f) => std :: io :: BufWriter :: new (f) , Err (e) => panic ! ("Error creating qlog file attempted path was {path:?}: {e}") , } }
};
}
