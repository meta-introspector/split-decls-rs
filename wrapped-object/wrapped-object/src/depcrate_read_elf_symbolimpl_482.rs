// Generated macro for impl_482 (impl)
macro_rules! Depcrate_read_elf_symbolimpl_482 {
() => {
// Module: crate::read::elf::symbol
// Provides: {"impl_482"}
// Dependencies: {}
impl < 'data , 'file , Elf : FileHeader , R : ReadRef < 'data > > ElfSymbol < 'data , 'file , Elf , R > { # [doc = " Get the endianness of the ELF file."] pub fn endian (& self) -> Elf :: Endian { self . endian } # [doc = " Return a reference to the raw symbol structure."] # [inline] # [deprecated (note = "Use `elf_symbol` instead")] pub fn raw_symbol (& self) -> & 'data Elf :: Sym { self . symbol } # [doc = " Get the raw ELF symbol structure."] pub fn elf_symbol (& self) -> & 'data Elf :: Sym { self . symbol } }
};
}
