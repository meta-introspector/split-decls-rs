// Generated macro for merge_modes (function)
macro_rules! Depcrate_tree_functionmerge_modes {
() => {
// Module: crate::tree::function
// Provides: {"merge_modes"}
// Dependencies: {}
# [doc = " Allows equal modes or prefers executables bits in case of blobs"] # [doc = ""] # [doc = " Note that this is often not correct as the previous mode of each side should be taken into account so that:"] # [doc = ""] # [doc = " on | on = on"] # [doc = " off | off = off"] # [doc = " on | off || off | on = conflict"] fn merge_modes (a : EntryMode , b : EntryMode) -> Option < EntryMode > { match (a . kind () , b . kind ()) { (_ , _) if a == b => Some (a) , (EntryKind :: BlobExecutable , EntryKind :: BlobExecutable | EntryKind :: Blob) | (EntryKind :: Blob , EntryKind :: BlobExecutable) => Some (EntryKind :: BlobExecutable . into ()) , _ => None , } }
};
}
