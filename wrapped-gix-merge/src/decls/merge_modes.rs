macro_rules! merge_modes {
    () => {
        # [doc = " Allows equal modes or prefers executables bits in case of blobs"] # [doc = ""] # [doc = " Note that this is often not correct as the previous mode of each side should be taken into account so that:"] # [doc = ""] # [doc = " on | on = on"] # [doc = " off | off = off"] # [doc = " on | off || off | on = conflict"] fn merge_modes (a : EntryMode , b : EntryMode) -> Option < EntryMode > { match (a . kind () , b . kind ()) { (_ , _) if a == b => Some (a) , (EntryKind :: BlobExecutable , EntryKind :: BlobExecutable | EntryKind :: Blob) | (EntryKind :: Blob , EntryKind :: BlobExecutable) => Some (EntryKind :: BlobExecutable . into ()) , _ => None , } }
    };
}

merge_modes!();