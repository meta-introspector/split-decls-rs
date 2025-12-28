macro_rules! deps {
    () => {
        File!();
        Offset!();
        EntryRange!();
        Version!();
        Kind!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        # [doc = " Information about the pack data file itself"] impl File { # [doc = " The pack data version of this file"] pub fn version (& self) -> Version { self . version } # [doc = " The number of objects stored in this pack data file"] pub fn num_objects (& self) -> u32 { self . num_objects } # [doc = " The length of all mapped data, including the pack header and the pack trailer"] pub fn data_len (& self) -> usize { self . data . len () } # [doc = " The kind of hash we use internally."] pub fn object_hash (& self) -> gix_hash :: Kind { self . object_hash } # [doc = " The position of the byte one past the last pack entry, or in other terms, the first byte of the trailing hash."] pub fn pack_end (& self) -> usize { self . data . len () - self . hash_len } # [doc = " The path to the pack data file on disk"] pub fn path (& self) -> & Path { & self . path } # [doc = " Returns the pack data at the given slice if its range is contained in the mapped pack data"] pub fn entry_slice (& self , slice : EntryRange) -> Option < & [u8] > { let entry_end : usize = slice . end . try_into () . expect ("end of pack fits into usize") ; let entry_start = slice . start as usize ; self . data . get (entry_start .. entry_end) } # [doc = " Returns the CRC32 of the pack data indicated by `pack_offset` and the `size` of the mapped data."] # [doc = ""] # [doc = " _Note:_ finding the right size is only possible by decompressing"] # [doc = " the pack entry beforehand, or by using the (to be sorted) offsets stored in an index file."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If `pack_offset` or `size` are pointing to a range outside of the mapped pack data."] pub fn entry_crc32 (& self , pack_offset : Offset , size : usize) -> u32 { let pack_offset : usize = pack_offset . try_into () . expect ("pack_size fits into usize") ; gix_features :: hash :: crc32 (& self . data [pack_offset .. pack_offset + size]) } }
    };
}

impl_192!();