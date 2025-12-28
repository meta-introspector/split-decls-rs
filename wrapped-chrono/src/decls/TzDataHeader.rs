macro_rules! deps {
    () => {
        Header!();
    };
}

macro_rules! TzDataHeader {
    () => {
        deps!();
        # [doc = " Header of the `tzdata` file."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] struct TzDataHeader { version : [u8 ; 5] , index_offset : u32 , data_offset : u32 , zonetab_offset : u32 , }
    };
}

TzDataHeader!();