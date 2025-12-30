// Generated macro for impl_18 (impl)
macro_rules! Depcrate_rewrites_trackerimpl_18 {
() => {
// Module: crate::rewrites::tracker
// Provides: {"impl_18"}
// Dependencies: {}
impl < T : Change > Item < T > { fn location < 'a > (& self , backing : & 'a [u8]) -> & 'a BStr { backing [self . path . clone ()] . as_ref () } fn entry_mode_compatible (& self , other : EntryMode) -> bool { use EntryKind :: * ; matches ! ((other . kind () , self . change . entry_mode () . kind ()) , (Blob | BlobExecutable , Blob | BlobExecutable) | (Link , Link) | (Tree , Tree)) } fn is_source_for_destination_of (& self , kind : visit :: SourceKind , dest_item_mode : EntryMode) -> bool { self . entry_mode_compatible (dest_item_mode) && match kind { visit :: SourceKind :: Rename => ! self . emitted && matches ! (self . change . kind () , ChangeKind :: Deletion) , visit :: SourceKind :: Copy => { matches ! (self . change . kind () , ChangeKind :: Modification) } } } }
};
}
