macro_rules! deps {
    () => {
        Reader!();
        Error!();
        Result!();
        FileEntryFormat!();
    };
}

macro_rules! impl_430 {
    () => {
        deps!();
        impl FileEntryFormat { fn parse < R : Reader > (input : & mut R) -> Result < Vec < FileEntryFormat > > { let format_count = input . read_u8 () ? as usize ; let mut format = Vec :: with_capacity (format_count) ; let mut path_count = 0 ; for _ in 0 .. format_count { let content_type = input . read_uleb128 () ? ; let content_type = if content_type > u64 :: from (u16 :: MAX) { constants :: DwLnct (u16 :: MAX) } else { constants :: DwLnct (content_type as u16) } ; if content_type == constants :: DW_LNCT_path { path_count += 1 ; } let form = constants :: DwForm (input . read_uleb128_u16 () ?) ; format . push (FileEntryFormat { content_type , form }) ; } if path_count != 1 { return Err (Error :: MissingFileEntryFormatPath) ; } Ok (format) } }
    };
}

impl_430!();