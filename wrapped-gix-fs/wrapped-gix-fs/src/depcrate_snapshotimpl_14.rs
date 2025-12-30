// Generated macro for impl_14 (impl)
macro_rules! Depcrate_snapshotimpl_14 {
() => {
// Module: crate::snapshot
// Provides: {"impl_14"}
// Dependencies: {}
# [doc = " Lifecycle"] impl < T : std :: fmt :: Debug > FileSnapshot < T > { # [doc = " A way for users to create 'fake' snapshot from `value` that isn't actually linked to a file on disk."] # [doc = ""] # [doc = " This is useful if there are alternative ways of obtaining the contained instance as fallback to trying"] # [doc = " to read it from disk."] pub fn new (value : T) -> Self { FileSnapshot { value , modified : std :: time :: UNIX_EPOCH , } } }
};
}
