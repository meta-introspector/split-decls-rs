// Generated macro for SectionRelocationIteratorInternal (enum)
macro_rules! Depcrate_read_anySectionRelocationIteratorInternal {
() => {
// Module: crate::read::any
// Provides: {"SectionRelocationIteratorInternal"}
// Dependencies: {}
# [derive (Debug)] enum SectionRelocationIteratorInternal < 'data , 'file , R : ReadRef < 'data > > { # [cfg (feature = "coff")] Coff (coff :: CoffRelocationIterator < 'data , 'file , R >) , # [cfg (feature = "coff")] CoffBig (coff :: CoffBigRelocationIterator < 'data , 'file , R >) , # [cfg (feature = "elf")] Elf32 (elf :: ElfSectionRelocationIterator32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "elf")] Elf64 (elf :: ElfSectionRelocationIterator64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO32 (macho :: MachORelocationIterator32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO64 (macho :: MachORelocationIterator64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "pe")] Pe32 (pe :: PeRelocationIterator < 'data , 'file , R >) , # [cfg (feature = "pe")] Pe64 (pe :: PeRelocationIterator < 'data , 'file , R >) , # [cfg (feature = "wasm")] Wasm (wasm :: WasmRelocationIterator < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff32 (xcoff :: XcoffRelocationIterator32 < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff64 (xcoff :: XcoffRelocationIterator64 < 'data , 'file , R >) , }
};
}
