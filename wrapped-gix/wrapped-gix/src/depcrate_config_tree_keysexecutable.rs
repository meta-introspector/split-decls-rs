// Generated macro for Executable (type)
macro_rules! Depcrate_config_tree_keysExecutable {
() => {
// Module: crate::config::tree::keys
// Provides: {"Executable"}
// Dependencies: {}
# [doc = " A key that represents an executable program as identified by name or path."] # [doc = ""] # [doc = " Once obtained with [trusted_program()](crate::config::Snapshot::trusted_program())"] # [doc = " one can run it with [command::prepare()](gix_command::prepare), possibly after"] # [doc = " [obtaining](crate::Repository::command_context) and [setting](gix_command::Prepare::with_context)"] # [doc = " a git [command context](gix_command::Context) (depending on the commands needs)."] pub type Executable = Any < validate :: Executable > ;
};
}
