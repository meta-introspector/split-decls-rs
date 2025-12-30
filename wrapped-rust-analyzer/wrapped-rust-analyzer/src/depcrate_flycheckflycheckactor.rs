// Generated macro for FlycheckActor (struct)
macro_rules! Depcrate_flycheckFlycheckActor {
() => {
// Module: crate::flycheck
// Provides: {"FlycheckActor"}
// Dependencies: {}
# [doc = " A [`FlycheckActor`] is a single check instance of a workspace."] struct FlycheckActor { # [doc = " The workspace id of this flycheck instance."] id : usize , generation : DiagnosticsGeneration , sender : Sender < FlycheckMessage > , config : FlycheckConfig , manifest_path : Option < AbsPathBuf > , ws_target_dir : Option < Utf8PathBuf > , # [doc = " Either the workspace root of the workspace we are flychecking,"] # [doc = " or the project root of the project."] root : Arc < AbsPathBuf > , sysroot_root : Option < AbsPathBuf > , scope : FlycheckScope , # [doc = " CargoHandle exists to wrap around the communication needed to be able to"] # [doc = " run `cargo check` without blocking. Currently the Rust standard library"] # [doc = " doesn't provide a way to read sub-process output without blocking, so we"] # [doc = " have to wrap sub-processes output handling in a thread and pass messages"] # [doc = " back over a channel."] command_handle : Option < CommandHandle < CargoCheckMessage > > , # [doc = " The receiver side of the channel mentioned above."] command_receiver : Option < Receiver < CargoCheckMessage > > , diagnostics_cleared_for : FxHashSet < Arc < PackageId > > , diagnostics_received : DiagnosticsReceived , }
};
}
