// Generated macro for run (function)
macro_rules! Depcrate_supportrun {
() => {
// Module: crate::support
// Provides: {"run"}
// Dependencies: {}
# [doc = " Attempts to run an executable, returning the `stdout` and `stderr` output if"] # [doc = " successful."] fn run (executable : & str , arguments : & [& str]) -> Result < (String , String) , String > { Command :: new (executable) . args (arguments) . output () . map (| o | { let stdout = String :: from_utf8_lossy (& o . stdout) . into_owned () ; let stderr = String :: from_utf8_lossy (& o . stderr) . into_owned () ; (stdout , stderr) }) . map_err (| e | format ! ("could not run executable `{}`: {}" , executable , e)) }
};
}
