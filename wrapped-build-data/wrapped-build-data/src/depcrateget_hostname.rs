// Generated macro for get_hostname (function)
macro_rules! Depcrateget_hostname {
() => {
// Module: crate
// Provides: {"get_hostname"}
// Dependencies: {}
# [doc = " Gets the name of the current machine."] # [doc = ""] # [doc = " Cargo doesn't pass the `HOSTNAME` env var to build scripts."] # [doc = " Uses the `hostname` command."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to execute the `hostname` command."] pub fn get_hostname () -> Result < String , String > { exec ("hostname" , & []) }
};
}
