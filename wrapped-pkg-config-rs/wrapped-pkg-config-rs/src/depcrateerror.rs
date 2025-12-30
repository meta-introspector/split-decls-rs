// Generated macro for Error (enum)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Represents all reasons `pkg-config` might not succeed or be run at all."] pub enum Error { # [doc = " Aborted because of `*_NO_PKG_CONFIG` environment variable."] # [doc = ""] # [doc = " Contains the name of the responsible environment variable."] EnvNoPkgConfig (String) , # [doc = " Detected cross compilation without a custom sysroot."] # [doc = ""] # [doc = " Ignore the error with `PKG_CONFIG_ALLOW_CROSS=1`,"] # [doc = " which may let `pkg-config` select libraries"] # [doc = " for the host's architecture instead of the target's."] CrossCompilation , # [doc = " Failed to run `pkg-config`."] # [doc = ""] # [doc = " Contains the command and the cause."] Command { command : String , cause : io :: Error } , # [doc = " `pkg-config` did not exit successfully after probing a library."] # [doc = ""] # [doc = " Contains the command and output."] Failure { command : String , output : Output } , # [doc = " `pkg-config` did not exit successfully on the first attempt to probe a library."] # [doc = ""] # [doc = " Contains the command and output."] ProbeFailure { name : String , command : String , output : Output , } , # [doc (hidden)] __Nonexhaustive , }
};
}
