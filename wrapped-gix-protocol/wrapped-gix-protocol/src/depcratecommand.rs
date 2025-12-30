// Generated macro for Command (enum)
macro_rules! DepcrateCommand {
() => {
// Module: crate
// Provides: {"Command"}
// Dependencies: {}
# [doc = " A selector for V2 commands to invoke on the server for purpose of pre-invocation validation."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub enum Command { # [doc = " List references."] LsRefs , # [doc = " Fetch a pack."] Fetch , }
};
}
