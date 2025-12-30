// Generated macro for Relocation (struct)
macro_rules! Depcrate_write_coff_writerRelocation {
() => {
// Module: crate::write::coff::writer
// Provides: {"Relocation"}
// Dependencies: {}
# [doc = " Native endian version of [`pe::ImageRelocation`]."] # [allow (missing_docs)] # [derive (Debug , Default , Clone)] pub struct Relocation { pub virtual_address : u32 , pub symbol : u32 , pub typ : u16 , }
};
}
