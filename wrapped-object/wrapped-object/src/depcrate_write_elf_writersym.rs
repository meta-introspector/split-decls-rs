// Generated macro for Sym (struct)
macro_rules! Depcrate_write_elf_writerSym {
() => {
// Module: crate::write::elf::writer
// Provides: {"Sym"}
// Dependencies: {}
# [doc = " Native endian version of [`elf::Sym64`]."] # [allow (missing_docs)] # [derive (Debug , Clone)] pub struct Sym { pub name : Option < StringId > , pub section : Option < SectionIndex > , pub st_info : u8 , pub st_other : u8 , pub st_shndx : u16 , pub st_value : u64 , pub st_size : u64 , }
};
}
