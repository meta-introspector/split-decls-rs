// Generated macro for visit_file_hashes (function)
macro_rules! Depcrate_manifestvisit_file_hashes {
() => {
// Module: crate::manifest
// Provides: {"visit_file_hashes"}
// Dependencies: {}
pub (crate) fn visit_file_hashes (manifest : & mut Manifest , mut f : impl FnMut (& mut FileHash)) { for pkg in manifest . pkg . values_mut () { for target in pkg . target . values_mut () { if let Some (hash) = & mut target . hash { f (hash) ; } if let Some (hash) = & mut target . xz_hash { f (hash) ; } } } for artifact in manifest . artifacts . values_mut () { for target in artifact . target . values_mut () { for file in target { f (& mut file . hash_sha256) ; } } } }
};
}
