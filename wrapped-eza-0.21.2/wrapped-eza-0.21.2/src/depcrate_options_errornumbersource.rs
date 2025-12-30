// Generated macro for NumberSource (enum)
macro_rules! Depcrate_options_errorNumberSource {
() => {
// Module: crate::options::error
// Provides: {"NumberSource"}
// Dependencies: {}
# [doc = " The source of a string that failed to be parsed as a number."] # [derive (PartialEq , Eq , Debug)] pub enum NumberSource { # [doc = " It came... from a command-line argument!"] Arg (& 'static Arg) , # [doc = " It came... from the environment!"] Env (& 'static str) , }
};
}
