// Generated macro for Entry (struct)
macro_rules! Depcrate_index_accessEntry {
() => {
// Module: crate::index::access
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " Represents an entry within a pack index file, effectively mapping object [`IDs`][gix_hash::ObjectId] to pack data file locations."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Entry { # [doc = " The ID of the object"] pub oid : gix_hash :: ObjectId , # [doc = " The offset to the object's header in the pack data file"] pub pack_offset : data :: Offset , # [doc = " The CRC32 hash over all bytes of the pack data entry."] # [doc = ""] # [doc = " This can be useful for direct copies of pack data entries from one pack to another with insurance there was no bit rot."] # [doc = " _Note_: Only available in index version 2 or newer"] pub crc32 : Option < u32 > , }
};
}
