macro_rules! deps {
    () => {
        FileMode!();
        Tree!();
        DiffFile!();
        Oid!();
        Blob!();
        Binding!();
        Commit!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        impl < 'a > DiffFile < 'a > { # [doc = " Returns the Oid of this item."] # [doc = ""] # [doc = " If this entry represents an absent side of a diff (e.g. the `old_file`"] # [doc = " of a `Added` delta), then the oid returned will be zeroes."] pub fn id (& self) -> Oid { unsafe { Binding :: from_raw (& (* self . raw) . id as * const _) } } # [doc = " Returns the path, in bytes, of the entry relative to the working"] # [doc = " directory of the repository."] pub fn path_bytes (& self) -> Option < & 'a [u8] > { static FOO : () = () ; unsafe { crate :: opt_bytes (& FOO , (* self . raw) . path) } } # [doc = " Returns the path of the entry relative to the working directory of the"] # [doc = " repository."] pub fn path (& self) -> Option < & 'a Path > { self . path_bytes () . map (util :: bytes2path) } # [doc = " Returns the size of this entry, in bytes"] pub fn size (& self) -> u64 { unsafe { (* self . raw) . size as u64 } } # [doc = " Returns `true` if file(s) are treated as binary data."] pub fn is_binary (& self) -> bool { unsafe { (* self . raw) . flags & raw :: GIT_DIFF_FLAG_BINARY as u32 != 0 } } # [doc = " Returns `true` if file(s) are treated as text data."] pub fn is_not_binary (& self) -> bool { unsafe { (* self . raw) . flags & raw :: GIT_DIFF_FLAG_NOT_BINARY as u32 != 0 } } # [doc = " Returns `true` if `id` value is known correct."] pub fn is_valid_id (& self) -> bool { unsafe { (* self . raw) . flags & raw :: GIT_DIFF_FLAG_VALID_ID as u32 != 0 } } # [doc = " Returns `true` if file exists at this side of the delta."] pub fn exists (& self) -> bool { unsafe { (* self . raw) . flags & raw :: GIT_DIFF_FLAG_EXISTS as u32 != 0 } } # [doc = " Returns file mode."] pub fn mode (& self) -> FileMode { match unsafe { (* self . raw) . mode . into () } { raw :: GIT_FILEMODE_UNREADABLE => FileMode :: Unreadable , raw :: GIT_FILEMODE_TREE => FileMode :: Tree , raw :: GIT_FILEMODE_BLOB => FileMode :: Blob , raw :: GIT_FILEMODE_BLOB_GROUP_WRITABLE => FileMode :: BlobGroupWritable , raw :: GIT_FILEMODE_BLOB_EXECUTABLE => FileMode :: BlobExecutable , raw :: GIT_FILEMODE_LINK => FileMode :: Link , raw :: GIT_FILEMODE_COMMIT => FileMode :: Commit , mode => panic ! ("unknown mode: {}" , mode) , } } }
    };
}

impl_333!()