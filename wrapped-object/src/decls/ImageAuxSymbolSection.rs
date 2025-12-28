macro_rules! deps {
    () => {
        U16Bytes!();
        ImageSymbol!();
        U32Bytes!();
        ImageSymbolEx!();
    };
}

macro_rules! ImageAuxSymbolSection {
    () => {
        deps!();
        # [doc = " Auxiliary symbol format 5: sections."] # [doc = ""] # [doc = " Used for both `ImageSymbol` and `ImageSymbolEx` (with padding)."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageAuxSymbolSection { # [doc = " section length"] pub length : U32Bytes < LE > , # [doc = " number of relocation entries"] pub number_of_relocations : U16Bytes < LE > , # [doc = " number of line numbers"] pub number_of_linenumbers : U16Bytes < LE > , # [doc = " checksum for communal"] pub check_sum : U32Bytes < LE > , # [doc = " section number to associate with"] pub number : U16Bytes < LE > , # [doc = " communal selection type"] pub selection : u8 , pub reserved : u8 , # [doc = " high bits of the section number"] pub high_number : U16Bytes < LE > , }
    };
}

ImageAuxSymbolSection!();