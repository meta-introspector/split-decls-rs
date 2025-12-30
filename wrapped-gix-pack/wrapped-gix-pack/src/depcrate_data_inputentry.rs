// Generated macro for Entry (struct)
macro_rules! Depcrate_data_inputEntry {
() => {
// Module: crate::data::input
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " An item of the iteration produced by [`BytesToEntriesIter`]"] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Entry { # [doc = " The header of a pack entry"] pub header : crate :: data :: entry :: Header , # [doc = " The amount of bytes used to encode the `header`. `pack_offset + header_size` is the beginning of"] # [doc = " the compressed data in the pack."] pub header_size : u16 , # [doc = " The first byte of the entry at which the `header` can be read."] pub pack_offset : u64 , # [doc = " The bytes consumed while producing `decompressed`"] # [doc = " These do not contain the header, which makes it possible to easily replace a RefDelta with offset deltas"] # [doc = " when resolving thin packs."] # [doc = " Depends on `CompressionMode` when the iterator is initialized."] pub compressed : Option < Vec < u8 > > , # [doc = " The amount of bytes the compressed portion of the entry takes, i.e. the portion behind the header."] pub compressed_size : u64 , # [doc = " The CRC32 over the complete entry, that is encoded header and compressed object data."] # [doc = " Depends on `CompressionMode` when the iterator is initialized"] pub crc32 : Option < u32 > , # [doc = " The amount of decompressed bytes of the entry."] pub decompressed_size : u64 , # [doc = " Set for the last object in the iteration, providing the hash over all bytes of the iteration"] # [doc = " for use as trailer in a pack or to verify it matches the trailer."] pub trailer : Option < gix_hash :: ObjectId > , }
};
}
