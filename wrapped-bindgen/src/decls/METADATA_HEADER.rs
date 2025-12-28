macro_rules! METADATA_HEADER {
    () => {
        # [repr (C)] # [derive (Default)] struct METADATA_HEADER { signature : u32 , major_version : u16 , minor_version : u16 , reserved : u32 , length : u32 , version : [u8 ; 20] , flags : u16 , streams : u16 , }
    };
}

METADATA_HEADER!();