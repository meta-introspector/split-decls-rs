// Generated macro for OpenError (enum)
macro_rules! DepcrateOpenError {
() => {
// Module: crate
// Provides: {"OpenError"}
// Dependencies: {}
# [doc = " An error type representing the failure to open a path. Possibly returned by the [`open`]"] # [doc = " function."] # [non_exhaustive] # [derive (Debug)] pub enum OpenError { # [doc = " An IO error occurred."] Io (io :: Error) , # [doc = " There was an error spawning command(s)."] Spawn { # [doc = " The command(s) that failed to spawn."] cmds : String , # [doc = " The underlying error."] source : io :: Error , } , # [doc = " A command exited with a non-zero exit status."] ExitStatus { # [doc = " A string that identifies the command."] cmd : & 'static str , # [doc = " The failed process's exit status."] status : ExitStatus , # [doc = " Anything the process wrote to stderr."] stderr : String , } , }
};
}
