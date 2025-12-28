macro_rules! deps {
    () => {
        U16Bytes!();
        U32Bytes!();
        I32Bytes!();
    };
}

macro_rules! ImageSymbolEx {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageSymbolEx { # [doc = " If first 4 bytes are 0, then second 4 bytes are offset into string table."] pub name : [u8 ; 8] , pub value : U32Bytes < LE > , pub section_number : I32Bytes < LE > , pub typ : U16Bytes < LE > , pub storage_class : u8 , pub number_of_aux_symbols : u8 , }
    };
}

ImageSymbolEx!();