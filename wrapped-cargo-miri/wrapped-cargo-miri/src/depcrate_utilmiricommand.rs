// Generated macro for MiriCommand (enum)
macro_rules! Depcrate_utilMiriCommand {
() => {
// Module: crate::util
// Provides: {"MiriCommand"}
// Dependencies: {}
# [derive (Clone , Debug)] pub enum MiriCommand { # [doc = " Our own special 'setup' command."] Setup , # [doc = " A command to be forwarded to cargo."] Forward (String) , # [doc = " Clean the miri cache"] Clean , }
};
}
