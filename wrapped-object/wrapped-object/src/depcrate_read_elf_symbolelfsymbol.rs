// Generated macro for ElfSymbol (struct)
macro_rules! Depcrate_read_elf_symbolElfSymbol {
() => {
// Module: crate::read::elf::symbol
// Provides: {"ElfSymbol"}
// Dependencies: {}
# [doc = " A symbol in an [`ElfFile`](super::ElfFile)."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSymbol`] trait implementation."] # [derive (Debug , Clone , Copy)] pub struct ElfSymbol < 'data , 'file , Elf , R = & 'data [u8] > where Elf : FileHeader , R : ReadRef < 'data > , { pub (super) endian : Elf :: Endian , pub (super) symbols : & 'file SymbolTable < 'data , Elf , R > , pub (super) index : SymbolIndex , pub (super) symbol : & 'data Elf :: Sym , }
};
}
