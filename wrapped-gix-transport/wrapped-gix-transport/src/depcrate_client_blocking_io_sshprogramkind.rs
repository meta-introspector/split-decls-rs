// Generated macro for ProgramKind (enum)
macro_rules! Depcrate_client_blocking_io_sshProgramKind {
() => {
// Module: crate::client::blocking_io::ssh
// Provides: {"ProgramKind"}
// Dependencies: {}
# [doc = " The kind of SSH programs we have built-in support for."] # [doc = ""] # [doc = " Various different programs exists with different capabilities, and we have a few built in."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum ProgramKind { # [doc = " The standard linux ssh program"] Ssh , # [doc = " The `(plink|putty).exe` binaries, typically only on windows."] Plink , # [doc = " The `putty.exe` binary, typically only on windows."] Putty , # [doc = " The `tortoiseplink.exe` binary, only on windows."] TortoisePlink , # [doc = " A minimal ssh client that supports on options."] Simple , }
};
}
