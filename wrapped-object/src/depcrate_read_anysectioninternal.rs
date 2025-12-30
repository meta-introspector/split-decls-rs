// Generated macro for SectionInternal (enum)
macro_rules! Depcrate_read_anySectionInternal {
() => {
// Module: crate::read::any
// Provides: {"SectionInternal"}
// Dependencies: {}
enum SectionInternal < 'data , 'file , R : ReadRef < 'data > > { # [cfg (feature = "coff")] Coff (coff :: CoffSection < 'data , 'file , R >) , # [cfg (feature = "coff")] CoffBig (coff :: CoffBigSection < 'data , 'file , R >) , # [cfg (feature = "elf")] Elf32 (elf :: ElfSection32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "elf")] Elf64 (elf :: ElfSection64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO32 (macho :: MachOSection32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO64 (macho :: MachOSection64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "pe")] Pe32 (pe :: PeSection32 < 'data , 'file , R >) , # [cfg (feature = "pe")] Pe64 (pe :: PeSection64 < 'data , 'file , R >) , # [cfg (feature = "wasm")] Wasm (wasm :: WasmSection < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff32 (xcoff :: XcoffSection32 < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff64 (xcoff :: XcoffSection64 < 'data , 'file , R >) , }
};
}
