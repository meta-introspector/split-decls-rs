macro_rules! deps {
    () => {
        Entry!();
        Id!();
    };
}

macro_rules! fanout {
    () => {
        deps!();
        # [doc = " Information for the chunk with the fanout table"] pub mod fanout { use crate :: multi_index ; # [doc = " The size of the fanout table"] pub const SIZE : usize = 4 * 256 ; # [doc = " The id uniquely identifying the fanout table."] pub const ID : gix_chunk :: Id = * b"OIDF" ; # [doc = " Decode the fanout table contained in `chunk`, or return `None` if it didn't have the expected size."] pub fn from_bytes (chunk : & [u8]) -> Option < [u32 ; 256] > { if chunk . len () != SIZE { return None ; } let mut out = [0 ; 256] ; for (c , f) in chunk . chunks_exact (4) . zip (out . iter_mut ()) { * f = u32 :: from_be_bytes (c . try_into () . unwrap ()) ; } out . into () } # [doc = " Write the fanout for the given entries, which must be sorted by oid"] pub (crate) fn write (sorted_entries : & [multi_index :: write :: Entry] , out : & mut dyn std :: io :: Write ,) -> std :: io :: Result < () > { let fanout = crate :: index :: encode :: fanout (& mut sorted_entries . iter () . map (| e | e . id . first_byte ())) ; for value in fanout . iter () { out . write_all (& value . to_be_bytes ()) ? ; } Ok (()) } }
    };
}

fanout!();