// Generated macro for copy_file (function)
macro_rules! Depcrate_utils_iocopy_file {
() => {
// Module: crate::utils::io
// Provides: {"copy_file"}
// Dependencies: {}
pub fn copy_file < S : AsRef < Path > , D : AsRef < Path > > (src : S , dst : D) -> anyhow :: Result < () > { log :: info ! ("Copying file {} to {}" , src . as_ref () . display () , dst . as_ref () . display ()) ; std :: fs :: copy (src . as_ref () , dst . as_ref ()) ? ; Ok (()) }
};
}
