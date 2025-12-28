macro_rules! tar_entry_type {
    () => {
        # [cfg (any (feature = "tar" , feature = "tar_gz"))] fn tar_entry_type (mode : gix_object :: tree :: EntryMode) -> tar :: EntryType { use gix_object :: tree :: EntryKind ; use tar :: EntryType ; match mode . kind () { EntryKind :: Tree | EntryKind :: Commit => EntryType :: Directory , EntryKind :: Blob => EntryType :: Regular , EntryKind :: BlobExecutable => EntryType :: Regular , EntryKind :: Link => EntryType :: Link , } }
    };
}

tar_entry_type!()