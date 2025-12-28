macro_rules! deps {
    () => {
        U32!();
        U16!();
    };
}

macro_rules! ImageFileHeader {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageFileHeader { pub machine : U16 < LE > , pub number_of_sections : U16 < LE > , pub time_date_stamp : U32 < LE > , pub pointer_to_symbol_table : U32 < LE > , pub number_of_symbols : U32 < LE > , pub size_of_optional_header : U16 < LE > , pub characteristics : U16 < LE > , }
    };
}

ImageFileHeader!()