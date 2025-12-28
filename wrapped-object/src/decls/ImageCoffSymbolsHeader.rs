macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageCoffSymbolsHeader {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageCoffSymbolsHeader { pub number_of_symbols : U32 < LE > , pub lva_to_first_symbol : U32 < LE > , pub number_of_linenumbers : U32 < LE > , pub lva_to_first_linenumber : U32 < LE > , pub rva_to_first_byte_of_code : U32 < LE > , pub rva_to_last_byte_of_code : U32 < LE > , pub rva_to_first_byte_of_data : U32 < LE > , pub rva_to_last_byte_of_data : U32 < LE > , }
    };
}

ImageCoffSymbolsHeader!();