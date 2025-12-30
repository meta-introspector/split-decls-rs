// Generated macro for impl_478 (impl)
macro_rules! Depcrate_read_elf_symbolimpl_478 {
() => {
// Module: crate::read::elf::symbol
// Provides: {"impl_478"}
// Dependencies: {}
impl < 'data , 'file , Elf : FileHeader , R : ReadRef < 'data > > Iterator for ElfSymbolIterator < 'data , 'file , Elf , R > { type Item = ElfSymbol < 'data , 'file , Elf , R > ; fn next (& mut self) -> Option < Self :: Item > { let index = self . index ; let symbol = self . symbols . symbols . get (index . 0) ? ; self . index . 0 += 1 ; Some (ElfSymbol { endian : self . endian , symbols : self . symbols , index , symbol , }) } }
};
}
