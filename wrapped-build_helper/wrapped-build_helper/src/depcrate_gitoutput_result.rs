// Generated macro for output_result (function)
macro_rules! Depcrate_gitoutput_result {
() => {
// Module: crate::git
// Provides: {"output_result"}
// Dependencies: {}
# [doc = " Runs a command and returns the output"] pub fn output_result (cmd : & mut Command) -> Result < String , String > { let output = match cmd . stderr (Stdio :: inherit ()) . output () { Ok (status) => status , Err (e) => return Err (format ! ("failed to run command: {cmd:?}: {e}")) , } ; if ! output . status . success () { return Err (format ! ("command did not execute successfully: {:?}\n\
             expected success, got: {}\n{}" , cmd , output . status , String :: from_utf8 (output . stderr) . map_err (| err | format ! ("{err:?}")) ?)) ; } String :: from_utf8 (output . stdout) . map_err (| err | format ! ("{err:?}")) }
};
}
