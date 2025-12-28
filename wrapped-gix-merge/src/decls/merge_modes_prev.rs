macro_rules! merge_modes_prev {
    () => {
        # [doc = " Use this version if there is a single common `prev` value for both `a` and `b` to detect"] # [doc = " if the mode was turned on or off."] fn merge_modes_prev (a : EntryMode , b : EntryMode , prev : EntryMode) -> Option < EntryMode > { match (a . kind () , b . kind ()) { (_ , _) if a == b => Some (a) , (a @ EntryKind :: BlobExecutable , b @ (EntryKind :: BlobExecutable | EntryKind :: Blob)) | (a @ EntryKind :: Blob , b @ EntryKind :: BlobExecutable) => { let prev = prev . kind () ; let changed = if a == prev { b } else { a } ; Some (match (prev , changed) { (EntryKind :: Blob , EntryKind :: BlobExecutable) => EntryKind :: BlobExecutable , (EntryKind :: BlobExecutable , EntryKind :: Blob) => EntryKind :: Blob , _ => unreachable ! ("upper match already assured we only deal with blobs") , } . into () ,) } _ => None , } }
    };
}

merge_modes_prev!()