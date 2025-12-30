// Generated macro for Command (struct)
macro_rules! Depcrate_blob_platform_mergeCommand {
() => {
// Module: crate::blob::platform::merge
// Provides: {"Command"}
// Dependencies: {}
# [doc = " The product of a [`PlatformRef::prepare_external_driver()`] operation."] # [doc = ""] # [doc = " This type allows to creation of [`std::process::Command`], ready to run, with `stderr` and `stdout` set to *inherit*,"] # [doc = " but `stdin` closed."] # [doc = " It's expected to leave its result in the file substituted at `current` which is then supposed to be read back from there."] # [allow (dead_code)] pub struct Command { # [doc = " The pre-configured command"] cmd : std :: process :: Command , # [doc = " A tempfile holding the *current* (ours) state of the resource."] current : gix_tempfile :: Handle < gix_tempfile :: handle :: Closed > , # [doc = " The path at which `current` is located, for reading the result back from later."] current_path : PathBuf , # [doc = " A tempfile holding the *ancestor* (base) state of the resource."] ancestor : gix_tempfile :: Handle < gix_tempfile :: handle :: Closed > , # [doc = " A tempfile holding the *other* (their) state of the resource."] other : gix_tempfile :: Handle < gix_tempfile :: handle :: Closed > , }
};
}
