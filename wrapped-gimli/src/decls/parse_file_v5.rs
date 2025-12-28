macro_rules! deps {
    () => {
        FileEntryFormat!();
        AttributeValue!();
        Result!();
        FileEntry!();
        Reader!();
        Encoding!();
    };
}

macro_rules! parse_file_v5 {
    () => {
        deps!();
        fn parse_file_v5 < R : Reader > (input : & mut R , encoding : Encoding , formats : & [FileEntryFormat] ,) -> Result < FileEntry < R > > { let mut path_name = None ; let mut directory_index = 0 ; let mut timestamp = 0 ; let mut size = 0 ; let mut md5 = [0 ; 16] ; let mut source = None ; for format in formats { let value = parse_attribute (input , encoding , format . form) ? ; match format . content_type { constants :: DW_LNCT_path => path_name = Some (value) , constants :: DW_LNCT_directory_index => { if let Some (value) = value . udata_value () { directory_index = value ; } } constants :: DW_LNCT_timestamp => { if let Some (value) = value . udata_value () { timestamp = value ; } } constants :: DW_LNCT_size => { if let Some (value) = value . udata_value () { size = value ; } } constants :: DW_LNCT_MD5 => { if let AttributeValue :: Block (mut value) = value { if value . len () . into_u64 () == 16 { md5 = value . read_u8_array () ? ; } } } constants :: DW_LNCT_LLVM_source => { source = Some (value) ; } _ => { } } } Ok (FileEntry { path_name : path_name . unwrap () , directory_index , timestamp , size , md5 , source , }) }
    };
}

parse_file_v5!()