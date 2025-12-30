// Generated macro for ReadDirUtf8 (struct)
macro_rules! DepcrateReadDirUtf8 {
() => {
// Module: crate
// Provides: {"ReadDirUtf8"}
// Dependencies: {}
# [doc = " Iterator over the entries in a directory."] # [doc = ""] # [doc = " This iterator is returned from [`Utf8Path::read_dir_utf8`] and will yield instances of"] # [doc = " <code>[io::Result]<[Utf8DirEntry]></code>. Through a [`Utf8DirEntry`] information like the entry's path"] # [doc = " and possibly other metadata can be learned."] # [doc = ""] # [doc = " The order in which this iterator returns entries is platform and filesystem"] # [doc = " dependent."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This [`io::Result`] will be an [`Err`] if there's some sort of intermittent"] # [doc = " IO error during iteration."] # [doc = ""] # [doc = " If a directory entry is not UTF-8, an [`io::Error`] is returned with the"] # [doc = " [`ErrorKind`](io::ErrorKind) set to [`InvalidData`][io::ErrorKind::InvalidData]"] # [doc = " and the payload set to a [`FromPathBufError`]."] # [derive (Debug)] pub struct ReadDirUtf8 { inner : fs :: ReadDir , }
};
}
