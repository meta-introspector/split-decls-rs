macro_rules! deps {
    () => {
        Version!();
        EntryDataMode!();
        Mode!();
        Kind!();
    };
}

macro_rules! BytesToEntriesIter {
    () => {
        deps!();
        # [doc = " An iterator over [`Entries`][input::Entry] in a byte stream."] # [doc = ""] # [doc = " The iterator used as part of [`Bundle::write_to_directory(…)`][crate::Bundle::write_to_directory()]."] pub struct BytesToEntriesIter < BR > { read : BR , decompressor : Decompress , offset : u64 , had_error : bool , version : crate :: data :: Version , objects_left : u32 , hash : Option < Hasher > , mode : input :: Mode , compressed : input :: EntryDataMode , compressed_buf : Option < Vec < u8 > > , hash_len : usize , object_hash : gix_hash :: Kind , }
    };
}

BytesToEntriesIter!()