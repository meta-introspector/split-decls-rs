// Generated macro for LinkingFailed (struct)
macro_rules! Depcrate_errorsLinkingFailed {
() => {
// Module: crate::errors
// Provides: {"LinkingFailed"}
// Dependencies: {}
pub (crate) struct LinkingFailed < 'a > { pub linker_path : & 'a Path , pub exit_status : ExitStatus , pub command : Command , pub escaped_output : String , pub verbose : bool , pub sysroot_dir : PathBuf , }
};
}
