macro_rules! deps {
    () => {
        Result!();
        ImageResourceDirectory!();
        ResourceDirectoryTable!();
        ImageResourceDirectoryEntry!();
    };
}

macro_rules! impl_722 {
    () => {
        deps!();
        impl < 'data > ResourceDirectoryTable < 'data > { fn parse (data : & 'data [u8] , offset : u32) -> Result < Self > { let mut offset = u64 :: from (offset) ; let header = data . read :: < pe :: ImageResourceDirectory > (& mut offset) . read_error ("Invalid resource table header") ? ; let entries_count = header . number_of_id_entries . get (LE) as usize + header . number_of_named_entries . get (LE) as usize ; let entries = data . read_slice :: < pe :: ImageResourceDirectoryEntry > (& mut offset , entries_count) . read_error ("Invalid resource table entries") ? ; Ok (Self { header , entries }) } }
    };
}

impl_722!()