// Generated macro for ReadDir (struct)
macro_rules! Depcrate_rootReadDir {
() => {
// Module: crate::root
// Provides: {"ReadDir"}
// Dependencies: {}
# [doc = " Iterator over the entries in a directory."] # [doc = ""] # [doc = " This iterator is returned from the [`Root::read_dir`] function and will"] # [doc = " yield instances of <code>[`io::Result`]<[`DirEntry`]></code>. Through a"] # [doc = " [`DirEntry`] information like the entry's path and possibly other metadata"] # [doc = " can be learned."] # [doc = ""] # [doc = " The order in which this iterator returns entries is platform and filesystem"] # [doc = " dependent."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This [`io::Result`] will be an [`Err`] if there's some sort of intermittent"] # [doc = " IO error during iteration."] pub struct ReadDir { inner : imp :: ReadDir , }
};
}
