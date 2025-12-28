macro_rules! deps {
    () => {
        TzDataIndex!();
        TzDataHeader!();
        TzDataIndexes!();
    };
}

macro_rules! impl_566 {
    () => {
        deps!();
        impl TzDataIndexes { # [doc = " Create a new `TzDataIndexes` from the `tzdata` file reader."] fn new < const ENTRY_LEN : usize > (mut reader : impl Read , header : & TzDataHeader) -> Result < Self > { let mut buf = vec ! [0 ; header . data_offset . saturating_sub (header . index_offset) as usize] ; reader . read_exact (& mut buf) ? ; Ok (TzDataIndexes { indexes : buf . chunks (ENTRY_LEN) . filter_map (| chunk | { from_bytes_until_nul (& chunk [.. TZ_NAME_LEN]) . map (| name | { let name = name . to_bytes () . to_vec () . into_boxed_slice () ; let offset = u32 :: from_be_bytes (chunk [TZ_NAME_LEN .. TZ_NAME_LEN + 4] . try_into () . unwrap () ,) ; let length = u32 :: from_be_bytes (chunk [TZ_NAME_LEN + 4 .. TZ_NAME_LEN + 8] . try_into () . unwrap () ,) ; TzDataIndex { name , offset , length } }) }) . collect () , }) } # [doc = " Find a timezone by name."] fn find_timezone (& self , timezone : & [u8]) -> Option < & TzDataIndex > { self . indexes . binary_search_by_key (& timezone , | x | & x . name) . map (| x | & self . indexes [x]) . ok () } # [doc = " Retrieve a chunk of timezone data by the index."] fn find_tzdata (& self , mut reader : impl Read + Seek , header : & TzDataHeader , index : & TzDataIndex ,) -> Result < Vec < u8 > > { reader . seek (SeekFrom :: Start (index . offset as u64 + header . data_offset as u64)) ? ; let mut buffer = vec ! [0 ; index . length as usize] ; reader . read_exact (& mut buffer) ? ; Ok (buffer) } }
    };
}

impl_566!()