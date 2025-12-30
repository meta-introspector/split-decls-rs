// Generated macro for SymbolTable (struct)
macro_rules! Depcrate_read_elf_symbolSymbolTable {
() => {
// Module: crate::read::elf::symbol
// Provides: {"SymbolTable"}
// Dependencies: {}
# [doc = " A table of symbol entries in an ELF file."] # [doc = ""] # [doc = " Also includes the string table used for the symbol names."] # [doc = ""] # [doc = " Returned by [`SectionTable::symbols`]."] # [derive (Debug , Clone , Copy)] pub struct SymbolTable < 'data , Elf : FileHeader , R = & 'data [u8] > where R : ReadRef < 'data > , { section : SectionIndex , string_section : SectionIndex , shndx_section : SectionIndex , symbols : & 'data [Elf :: Sym] , strings : StringTable < 'data , R > , shndx : & 'data [U32 < Elf :: Endian >] , }
};
}
