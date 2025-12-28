macro_rules! AuxSymbolSection {
    () => {
        # [doc = " Native endian version of [`pe::ImageAuxSymbolSection`]."] # [allow (missing_docs)] # [derive (Debug , Default , Clone)] pub struct AuxSymbolSection { pub length : u32 , # [doc = " This will automatically be clamped if there are more than 0xffff."] pub number_of_relocations : u32 , pub number_of_linenumbers : u16 , pub check_sum : u32 , pub number : u32 , pub selection : u8 , }
    };
}

AuxSymbolSection!();