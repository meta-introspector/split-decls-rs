// Generated macro for CommandOutput (struct)
macro_rules! Depcrate_utils_execCommandOutput {
() => {
// Module: crate::utils::exec
// Provides: {"CommandOutput"}
// Dependencies: {}
# [doc = " Represents the output of an executed process."] # [derive (Clone , PartialEq)] pub struct CommandOutput { status : CommandStatus , stdout : Option < Vec < u8 > > , stderr : Option < Vec < u8 > > , }
};
}
