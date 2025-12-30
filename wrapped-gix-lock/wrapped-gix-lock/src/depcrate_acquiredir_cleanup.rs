// Generated macro for dir_cleanup (function)
macro_rules! Depcrate_acquiredir_cleanup {
() => {
// Module: crate::acquire
// Provides: {"dir_cleanup"}
// Dependencies: {}
fn dir_cleanup (boundary : Option < PathBuf >) -> (ContainingDirectory , AutoRemove) { match boundary { None => (ContainingDirectory :: Exists , AutoRemove :: Tempfile) , Some (boundary_directory) => (ContainingDirectory :: CreateAllRaceProof (Default :: default ()) , AutoRemove :: TempfileAndEmptyParentDirectoriesUntil { boundary_directory } ,) , } }
};
}
