macro_rules! deps {
    () => {
        ReadDirUtf8!();
        Utf8PathBuf!();
    };
}

macro_rules! Utf8DirEntry {
    () => {
        deps!();
        # [doc = " Entries returned by the [`ReadDirUtf8`] iterator."] # [doc = ""] # [doc = " An instance of [`Utf8DirEntry`] represents an entry inside of a directory on the filesystem. Each"] # [doc = " entry can be inspected via methods to learn about the full path or possibly other metadata."] # [derive (Debug)] pub struct Utf8DirEntry { inner : fs :: DirEntry , path : Utf8PathBuf , }
    };
}

Utf8DirEntry!()