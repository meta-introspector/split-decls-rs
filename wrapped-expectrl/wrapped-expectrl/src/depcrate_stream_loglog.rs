// Generated macro for log (function)
macro_rules! Depcrate_stream_loglog {
() => {
// Module: crate::stream::log
// Provides: {"log"}
// Dependencies: {}
fn log (mut writer : impl Write , target : & str , data : & [u8]) { let _ = match std :: str :: from_utf8 (data) { Ok (data) => writeln ! (writer , "{}: {:?}" , target , data) , Err (..) => writeln ! (writer , "{}:(bytes): {:?}" , target , data) , } ; }
};
}
