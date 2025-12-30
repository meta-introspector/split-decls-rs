// Generated macro for Verdef (struct)
macro_rules! Depcrate_write_elf_writerVerdef {
() => {
// Module: crate::write::elf::writer
// Provides: {"Verdef"}
// Dependencies: {}
# [doc = " Information required for writing [`elf::Verdef`]."] # [allow (missing_docs)] # [derive (Debug , Clone)] pub struct Verdef { pub version : u16 , pub flags : u16 , pub index : u16 , pub aux_count : u16 , # [doc = " The name for the first [`elf::Verdaux`] entry."] pub name : StringId , }
};
}
