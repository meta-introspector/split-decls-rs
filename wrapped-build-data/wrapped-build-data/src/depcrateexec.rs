// Generated macro for exec (function)
macro_rules! Depcrateexec {
() => {
// Module: crate
// Provides: {"exec"}
// Dependencies: {}
# [doc = " Executes `cmd` with `args` as parameters, waits for it to exit, and"] # [doc = " returns its stdout, trimmed, and escaped with"] # [doc = " [`escape_ascii`](#method.escape_ascii)."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns a descriptive error string if it fails to execute the command"] # [doc = " or if the command exits with a non-zero status."] pub fn exec (cmd : impl AsRef < OsStr > , args : & [& str]) -> Result < String , String > { let output = std :: process :: Command :: new (cmd . as_ref ()) . args (args) . output () . map_err (| e | { format ! ("error executing '{} {}': {e}" , cmd . as_ref () . to_string_lossy () , args . join (" ") ,) }) ? ; if ! output . status . success () { return Err (format ! ("command '{} {}' failed: exit={} stdout='{}' stderr='{}'" , cmd . as_ref () . to_string_lossy () , args . join (" ") , output . status . code () . map_or_else (|| String :: from ("signal") , | c | c . to_string ()) , escape_ascii (output . stdout) , escape_ascii (output . stderr))) ; } let stdout = String :: from_utf8_lossy (& output . stdout) ; Ok (escape_ascii (stdout . trim ())) }
};
}
