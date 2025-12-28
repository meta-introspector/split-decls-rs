macro_rules! deps {
    () => {
        FileEntryFormat!();
        Result!();
        Encoding!();
        AttributeValue!();
        Reader!();
    };
}

macro_rules! parse_directory_v5 {
    () => {
        deps!();
        fn parse_directory_v5 < R : Reader > (input : & mut R , encoding : Encoding , formats : & [FileEntryFormat] ,) -> Result < AttributeValue < R > > { let mut path_name = None ; for format in formats { let value = parse_attribute (input , encoding , format . form) ? ; if format . content_type == constants :: DW_LNCT_path { path_name = Some (value) ; } } Ok (path_name . unwrap ()) }
    };
}

parse_directory_v5!();