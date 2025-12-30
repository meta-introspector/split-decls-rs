// Generated macro for Symbol (struct)
macro_rules! Depcrate_write_coff_writerSymbol {
() => {
// Module: crate::write::coff::writer
// Provides: {"Symbol"}
// Dependencies: {}
# [doc = " Native endian version of [`pe::ImageSymbol`]."] # [allow (missing_docs)] # [derive (Debug , Default , Clone)] pub struct Symbol { pub name : Name , pub value : u32 , pub section_number : u16 , pub typ : u16 , pub storage_class : u8 , pub number_of_aux_symbols : u8 , }
};
}
