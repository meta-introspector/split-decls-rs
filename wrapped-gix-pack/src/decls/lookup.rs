macro_rules! deps {
    () => {
        Kind!();
        Id!();
        Entry!();
    };
}

macro_rules! lookup {
    () => {
        deps!();
        # [doc = " Information about the oid lookup table."] pub mod lookup { use std :: ops :: Range ; use crate :: multi_index ; # [doc = " The id uniquely identifying the oid lookup table."] pub const ID : gix_chunk :: Id = * b"OIDL" ; # [doc = " Return the number of bytes needed to store the data on disk for the given amount of `entries`"] pub fn storage_size (entries : usize , object_hash : gix_hash :: Kind) -> u64 { (entries * object_hash . len_in_bytes ()) as u64 } pub (crate) fn write (sorted_entries : & [multi_index :: write :: Entry] , out : & mut dyn std :: io :: Write ,) -> std :: io :: Result < () > { for entry in sorted_entries { out . write_all (entry . id . as_slice ()) ? ; } Ok (()) } # [doc = " Return true if the size of the `offset` range seems to match for a `hash` of the given kind and the amount of objects."] pub fn is_valid (offset : & Range < usize > , hash : gix_hash :: Kind , num_objects : u32) -> bool { (offset . end - offset . start) / hash . len_in_bytes () == num_objects as usize } }
    };
}

lookup!()