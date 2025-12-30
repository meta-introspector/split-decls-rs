// Generated macro for utf8_stdout (function)
macro_rules! Depcrateutf8_stdout {
() => {
// Module: crate
// Provides: {"utf8_stdout"}
// Dependencies: {}
fn utf8_stdout (cmd : & mut Command) -> anyhow :: Result < String > { let output = cmd . output () . with_context (| | format ! ("{cmd:?} failed")) ? ; if ! output . status . success () { match String :: from_utf8 (output . stderr) { Ok (stderr) if ! stderr . is_empty () => { bail ! ("{:?} failed, {}\nstderr:\n{}" , cmd , output . status , stderr) } _ => bail ! ("{:?} failed, {}" , cmd , output . status) , } } let stdout = String :: from_utf8 (output . stdout) ? ; Ok (stdout . trim () . to_owned ()) }
};
}
