// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl VersionMeta { # [doc = " Returns the version metadata for `cmd`, which should be a `rustc` command."] pub fn for_command (mut cmd : Command) -> Result < VersionMeta > { let out = cmd . arg ("-vV") . output () . map_err (Error :: CouldNotExecuteCommand) ? ; if ! out . status . success () { return Err (Error :: CommandError { stdout : String :: from_utf8_lossy (& out . stdout) . into () , stderr : String :: from_utf8_lossy (& out . stderr) . into () , }) ; } version_meta_for (str :: from_utf8 (& out . stdout) ?) } }
};
}
