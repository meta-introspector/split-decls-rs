macro_rules! deps {
    () => {
        Entry!();
        Id!();
    };
}

macro_rules! offsets {
    () => {
        deps!();
        # [doc = " Information about the offsets table."] pub mod offsets { use std :: ops :: Range ; use crate :: multi_index ; # [doc = " The id uniquely identifying the offsets table."] pub const ID : gix_chunk :: Id = * b"OOFF" ; # [doc = " Return the amount of bytes needed to offset data for `entries`."] pub fn storage_size (entries : usize) -> u64 { (entries * (4 + 4)) as u64 } pub (crate) fn write (sorted_entries : & [multi_index :: write :: Entry] , large_offsets_needed : bool , out : & mut dyn std :: io :: Write ,) -> std :: io :: Result < () > { use crate :: index :: encode :: { HIGH_BIT , LARGE_OFFSET_THRESHOLD } ; let mut num_large_offsets = 0u32 ; for entry in sorted_entries { out . write_all (& entry . pack_index . to_be_bytes ()) ? ; let offset : u32 = if large_offsets_needed { if entry . pack_offset > LARGE_OFFSET_THRESHOLD { let res = num_large_offsets | HIGH_BIT ; num_large_offsets += 1 ; res } else { entry . pack_offset as u32 } } else { entry . pack_offset . try_into () . expect ("without large offsets, pack-offset fits u32") } ; out . write_all (& offset . to_be_bytes ()) ? ; } Ok (()) } # [doc = " Returns true if the `offset` range seems to match the size required for `num_objects`."] pub fn is_valid (offset : & Range < usize > , num_objects : u32) -> bool { let entry_size = 4 + 4 ; ((offset . end - offset . start) / num_objects as usize) == entry_size } }
    };
}

offsets!();