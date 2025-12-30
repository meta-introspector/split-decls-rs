// Generated macro for Link (struct)
macro_rules! Depcrate_extensionLink {
() => {
// Module: crate::extension
// Provides: {"Link"}
// Dependencies: {}
# [doc = " The link extension to track a shared index."] # [derive (Clone)] pub struct Link { # [doc = " The checksum of the shared index as last seen."] pub shared_index_checksum : gix_hash :: ObjectId , # [doc = " Bitmaps to tell us which entries to delete or replace."] pub bitmaps : Option < link :: Bitmaps > , }
};
}
